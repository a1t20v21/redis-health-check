## Steps to run the code
```bash
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release

export REDIS_ADDR=localhost:6379
export REDIS_PASSWORD=test123

./target/x86_64-unknown-linux-musl/release/redis-health-check <timeout in seconds>
```


