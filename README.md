Start with:
```
RUST_LOG=info cargo run -- -t <TOPIC> -g <GROUPID> -b 127.0.0.1:9092
```

As Docker
```
docker build -t IMAGENAME .
docker run --workdir /app IMAGENAME /app/target/release/consumetx -t <TOPIC> -g <GROUPID> -b 127.0.0.1:9092
```
