# Getting Started

* OAuth Scopes を設定します。
  * channels:read
  * chat:write
  * files:write
  * groups:read
  * im:read
  * im:write

* .settings.json を作成します。

```json
{
    "access_token": "xoxb-a-b-c"
}
```

* 実行します。

```sh
cargo run
```

# 2024-10-13
* WSAE 10014 「無効なポインター・アドレス～」 が出るようになった
* `cargo update` したら解消した。
