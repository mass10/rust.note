

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

    // 全角文字がある場合
    let zenkaku_aiueo: Vec<char> = "あいうえお".chars().collect::<Vec<char>>();
    println!("[{}]", &zenkaku_aiueo[..0].iter().collect::<String>());
    println!("[{}]", &zenkaku_aiueo[..1].iter().collect::<String>());
    println!("[{}]", &zenkaku_aiueo[..2].iter().collect::<String>());
    println!("[{}]", &zenkaku_aiueo[1..1].iter().collect::<String>());
    println!("[{}]", &zenkaku_aiueo[1..2].iter().collect::<String>());
    println!("[{}]", &zenkaku_aiueo[1..3].iter().collect::<String>());
}
