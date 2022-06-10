RUST_BACKTRACE=1 cargo unit-test
cargo clippy -- -D warnings
cargo schema --locked
