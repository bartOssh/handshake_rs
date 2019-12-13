# handshake_rs
AWS Lambda function for handshake of the day, based on given rule.


## Local Development

- To run local server on port 3000: `LAMBDA_ENV=false cargo run`

## Shipping to production.
- Run all tests: `cargo test`
- Set corresponding target in compiler `rustup target add x86_64-unknown-linux-musl`
- Compile `cargo build --release --target x86_64-unknown-linux-musl`
- Zip binary executable: `zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap`
- Upload no lambda and test, remember to set environment variable `LAMBDA_ENV=true`
