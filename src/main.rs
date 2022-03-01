use clap::{Command, Arg};
use log::{error, info};
mod tps;

use rdkafka::{Message, client::ClientContext, consumer::CommitMode, message::{BorrowedMessage}};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, ConsumerContext, Rebalance};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::util::get_rdkafka_version;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Command::new("consumer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .arg(Arg::new("brokers")
            .short('b')
            .long("brokers")
            .takes_value(true)
            .env("BROKERS")
            .default_value("localhost:9092")
        )
        .arg(Arg::new("topics")
            .short('t')
            .long("topics")
            .takes_value(true)
            .multiple_occurrences(true)
            .env("TOPICS")
            .default_values(&["test"])
        )
        .arg(Arg::new("group-id")
            .short('g')
            .long("group-id")
            .takes_value(true)
            .env("GROUP_ID")
            .default_value("gid")
        )
        .arg(
            Arg::new("destination")
                .short('d')
                .long("destination")
                .help("Destination topic")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let (ver_n, ver_s) = get_rdkafka_version();
    info!("rd_kafka_Version: 0x{:08x}, {}", ver_n, ver_s);

    let brokers = args.value_of("brokers").unwrap();
    let topics: Vec<&str> = args.values_of("topics").unwrap().collect();
    let group_id = args.value_of("group-id").unwrap();
    let destination = args.value_of("destination").unwrap();

    info!("Starting to consume & produce");
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
        
    consume(brokers, group_id, &topics, destination, producer).await;
}

struct LoggingContext;

impl ClientContext for LoggingContext {}

impl ConsumerContext for LoggingContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }
    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }
}



async fn consume(brokers: &str, group_id: &str, topics: &[&str], destination: &str, producer: &FutureProducer) {
    let consumer: StreamConsumer<LoggingContext> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(LoggingContext)
        .expect("Consumer creation failed");

    consumer
        .subscribe(topics)
        .expect(&format!("Failed to subscribe to topics {:?}", topics));

		let mut txs:Vec<(i64,i64)>=Vec::with_capacity(10);

    loop {
        let r = consumer.recv().await;
        match r {
            Err(e) => error!("Kafka error: {}", e),
            Ok(m) => {
                process_msg(&m,&mut txs,&destination,&producer);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        }
    }

}

fn process_msg(msg: &BorrowedMessage, txs: &mut Vec<(i64,i64)>,destination:&str,producer:&FutureProducer) {
    let payload = match msg.payload_view::<str>() {
        None => "",
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            error!("Error while deserializing payload: {:?}", e);
            ""
        }
    };
    let tx = payload.parse::<i64>().unwrap();
    if txs.len() > 9{
    	txs.remove(0);
    }
    txs.push((tx,msg.timestamp().to_millis().unwrap()));
    if txs.len() > 9{
    	//info!("\n-----------\n");
      //info!("{:#?}",txs);
	    let real_tps = tps::calc(txs);
    	//info!("TPS: {:#?}, producing to {}",real_tps,destination);
    	let key = msg.timestamp().to_millis().unwrap().to_string();
      producer.send_result(
          FutureRecord::to(destination)
              .payload(&real_tps.to_string())
              .key(&key),
      ).map_err(|err| println!("{:?}", err)).ok();
      
    };

    
}
