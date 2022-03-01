Start with:
```
RUST_LOG=info cargo run -- -t <TOPIC> -g <GROUPID>  -d <TOPIC DESTINATION> -b 127.0.0.1:9092 # Broker is optional 
```

As Docker
```
docker build -t app:debian .
docker run -ti --rm app:debian /app/kafkaconsumer -t <TOPIC> -g <GROUPID> -d <TOPIC DESTINATION> -b 127.0.0.1:9092
```

This program consumes the `sven_transaction_count` topic and produces tps to the `sven_tps` topic
