FROM rust
WORKDIR /app
RUN apt-get update && apt-get -y install cmake 
COPY . .
RUN cargo build --release
