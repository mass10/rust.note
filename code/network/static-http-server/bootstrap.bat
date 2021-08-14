@SETLOCAL
@ECHO OFF

cargo fmt
cargo run -- %*

