@SETLOCAL

@REM SET RUSTFLAGS="%RUSTFLAGS% -A dead_code"

@CALL cargo fmt
@CALL cargo run --release -- %*
