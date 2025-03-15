start:
  RUST_BACKTRACE=1 cargo run

start-prod:
  cargo clean
  cargo build --release
  ./target/release/zyst

fmt:
  cargo check
  cargo fmt
  cargo clippy

client:
  nc 127.0.0.1 6379

test-ut:
  RUST_BACKTRACE=1 cargo test --tests ut

test-inte:
  rm -rf ~/.local/share/zyst/appendonly.aof
  RUST_BACKTRACE=1 cargo test --tests inte -- --nocapture --test-threads=1

bench:
  cargo bench --bench my_benchmark

bench-graph:
  cargo flamegraph --bench my_benchmark

benchmark:
  redis-cli FLUSHDB
  redis-benchmark -t set,get,incr,lpush,rpush,lpop,rpop,hset,lpush,lrange_100,lrqnge_300,lrange_500,lrange_600 -n 100000 -q

bench-start:
  cargo flamegraph --bin zyst

bench-report:
  perf report --no-children --sort=dso | grep zyst
