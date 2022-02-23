use clap::{Command, Arg};
use log::{error, info};

use rdkafka::{Message, client::ClientContext, consumer::CommitMode, message::{BorrowedMessage, Headers}};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, ConsumerContext, Rebalance};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use rdkafka::topic_partition_list::{TopicPartitionList};
use rdkafka::util::get_rdkafka_version;

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
        .get_matches();

    let (ver_n, ver_s) = get_rdkafka_version();
    info!("rd_kafka_Version: 0x{:08x}, {}", ver_n, ver_s);

    let brokers = args.value_of("brokers").unwrap();
    let topics: Vec<&str> = args.values_of("topics").unwrap().collect();
    let group_id = args.value_of("group-id").unwrap();

    info!("Starting to consume");
    consume(brokers, group_id, &topics).await;
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

    fn commit_callback(&self, result: KafkaResult<()>, _: &TopicPartitionList) {
        info!("Commiting offsets: {:?}", result);
    }
}

async fn consume(brokers: &str, group_id: &str, topics: &[&str]) {
    let consumer: StreamConsumer<LoggingContext> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(LoggingContext)
        .expect("Consumer creation failed");

    consumer
        .subscribe(topics)
        .expect(&format!("Failed to subscribe to topics {:?}", topics));

    loop {
        let r = consumer.recv().await;
        match r {
            Err(e) => error!("Kafka error: {}", e),
            Ok(m) => {
                process_msg(&m);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        }
    }

}

fn process_msg(msg: &BorrowedMessage) {
    let payload = match msg.payload_view::<str>() {
        None => "",
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            error!("Error while deserializing payload: {:?}", e);
            ""
        }
    };

    info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
        msg.key(), payload, msg.topic(), msg.partition(), msg.offset(), msg.timestamp());
    info!("msg: {:?}", msg);

    if let Some(headers) = msg.headers() {
        for i in 0..headers.count() {
            let header = headers.get(i).unwrap();
            let header_key = header.0;
            let header_value = std::str::from_utf8(header.1).unwrap();
            info!("  Header '{}' : '{}'", header_key, header_value);
        }
    }
}
