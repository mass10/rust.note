fn diagnose(s: String) {
	println!("{}", s);
}

fn main() {
	let s = String::from("テスト");
    diagnose(s); // value moved heer
    diagnose(s); // value used here after move (COMPILATION ERROR)
}
