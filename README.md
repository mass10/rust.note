# rust.note

# Visual Studio Code

- 「nightly toolchain not installed. Install?」と言われたので入れた。その後 nightly を削除したら「RLS could not be started.」と言われた。

![画像](../../.images/RLS could not be started.png "名前")

# components

```
rustup component list
```

# toolchains

```
rustup toolchain list
```

# removing toolchain

```
rustup toolchain remove nightly
```

# Setting up rust code itself in Windows 10. (2018-05-07)

```
rustup component add rust-src
```

# Rust for Windows (2018-05-07)

rustup installer を実行。
=======
# Installing racer (2018-05-09)

シンタックスハイライトとかに使うもの？？ (Rust language support は Racer ではないぽい)

```
cargo install racer
```

# Rust 本体のソースファイルをセットアップする (2018-05-07)

```
rustup component add rust-src
```

# Installation on Ubuntu 16

```
curl https://sh.rustup.rs -sSf | sh
```

下記の質問には無言で `enter`

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
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


# updating Rust

```
rustup update
```

```
neu@ubuntu:~/workspace/rust.note/code/database$ rustup update
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2018-03-29, rust version 1.25.0 (84203cac6 2018-03-25)
info: downloading component 'rustc'
 55.2 MiB /  55.2 MiB (100 %)  11.0 MiB/s ETA:   0 s
info: downloading component 'rust-std'
 47.3 MiB /  47.3 MiB (100 %)  10.8 MiB/s ETA:   0 s
info: downloading component 'cargo'
info: downloading component 'rust-docs'
info: removing component 'rustc'
info: removing component 'rust-std'
info: removing component 'cargo'
info: removing component 'rust-docs'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: checking for self-updates
info: downloading self-update

  stable-x86_64-unknown-linux-gnu updated - rustc 1.25.0 (84203cac6 2018-03-25)
```
