@SETLOCAL
@CALL cargo fmt
@CALL cargo build --release
@COPY target\release\invis*.exe .
