# About

* Rust メモ

# cargo bench 時の最適化に注意 (2018-05-17)

`test::black_box` → cargo bench 実行時の最適化を回避

# RLS could not be started with Visual Studio Code. (2018-05-12)

「nightly toolchain not installed. Install?」と言われたので入れた。その後 nightly を削除したら「RLS could not be started.」と言われた。
ユーザー設定に `"rust-client.channel": "stable"` を入れて Visual Studio Code を再起動たら解消した。

# components

```bash
rustup component list
```

# toolchains
* システムにセットアップされている Rust システムのリスト
```bash
rustup toolchain list
```

# use toolchain stable (or nightly)
* デフォルトでどの Rust システムを使用するか
```bash
rustup default stable
rustup default nightly
```

# removing toolchain

```bash
rustup toolchain remove nightly
```

# install stable

```bash
rustup toolchain install stable
```

# Setting up rust code itself in Windows 10. (2018-05-07)

```bash
rustup component add rust-src
```

# Rust for Windows (2018-05-07)

rustup installer をみつけてきて実行します。

# Installing racer (2018-05-09)

シンタックスハイライトとかに使うもの？？ (Rust language support は Racer ではないぽい)

```bash
cargo install racer
```

# Rust 本体のソースファイルをセットアップする (2018-05-07)

```bash
rustup component add rust-src
```

# Installation on Ubuntu 16

```bash
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

```bash
rustc --version
```

# building hello.rs

```bash
rustc hello.rs
```

で Go よりもちょっと大きなバイナリ hello* ができる。

# Get started with Cargo.

```bash
cargo new testapp2 --bin
cd testapp2
cargo build
cargo run
```

または

```bash
mkdir testapp1
cd testapp1
cargo init --bin
cargo build
cargo run
```

# build

```bash
cargo build
```

or 

```bash
cargo build --release
```


# updating Rust

```bash
rustup update
```

```plain
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

# error: no default toolchain configured

```bash
rustup default stable
```

# error: toolchain 'stable-x86_64-unknown-linux-gnu' does not contain component 'rustfmt' for target 'x86_64-unknown-linux-gnu'

```bash
# try
rustup toolchain remove stable
rustup toolchain install stable

# then
rustup component add rustfmt
```

# To format Rust source

```bash
rustup component add rustfmt
cargo fmt
```

* rustfmt のコンフィギュレーション https://github.com/rust-lang/rustfmt/blob/master/Configurations.md#hard_tabs
