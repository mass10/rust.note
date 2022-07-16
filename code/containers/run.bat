@SETLOCAL

@ECHO OFF

CALL cargo fmt

SET X_TARGET_TASK=default
SET X_TARGET_TASK=performance

CALL cargo run --bin %X_TARGET_TASK%
