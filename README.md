# rust.note

# Installation on Ubuntu 16

```
curl https://sh.rustup.rs -sSf | sh
```

> Rust is installed now. Great!

といわれたら `~/.cargo/bin` を PATH に追加するように言われるので追加。
追加したらログアウト & ログイン

```
rustc --version
```

# building hello.rs

```
rustc hello.rs
```

で Go よりもちょっと大きなバイナリ hello* ができる。

# Get started with Cargo.

```
cargo new testapp2 --bin
cd testapp2
cargo build
cargo run
```

##### または

```
mkdir testapp1
cd testapp1
cargo init --bin
cargo build
cargo run
```

# build

```
cargo build
```

or 

```
cargo build --release
```
