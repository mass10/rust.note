# 文字列の扱い

```Rust
fn main() {
    // [AB]
    println!("[{}]", &"ABCDEFG"[0..2]);

    // [ABCDE]
    println!("[{}]", &"ABCDEFG"[0..5]);

    // [BC]
    println!("[{}]", &"ABCDEFG"[1..3]);

    // [C]
    println!("[{}]", &"ABCDEFG"[2..3]);

    // [EFG]
    println!("[{}]", &"ABCDEFG"[4..]);

    // [A]
    println!("[{}]", &"ABCDEFG"[..1]);
}
```
