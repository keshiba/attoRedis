[![Rust](https://github.com/keshiba/attoRedis/actions/workflows/build.yml/badge.svg)](https://github.com/keshiba/attoRedis/actions/workflows/build.yml)

# attoRedis

A quick implementation of redis. Mostly to learn Rust and Tokio

### How to run ?

1. Start the server
```shell
cargo run --package attoredis-server -- --port <port>
```

2. Start the client
```shell
cargo run --package attoredis-client -- --port <port>
```
