Start with:
```
RUST_LOG=info cargo run -- -t <TOPIC> -g <GROUPID> -b 127.0.0.1:9092
```

As Docker
```
docker build -t app:debian .
docker run -ti --rm app:debian /app/consumetx /app/target/release/consumetx -t <TOPIC> -g <GROUPID> -b 127.0.0.1:9092
```
