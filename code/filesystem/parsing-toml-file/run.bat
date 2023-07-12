@SETLOCAL

@CALL cargo fmt
@CALL cargo run -- settings.toml
