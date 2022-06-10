RUST_BACKTRACE=1 cargo unit-test -- --nocapture
cargo clippy -- -D warnings
cargo schema --locked
