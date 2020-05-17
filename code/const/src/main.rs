/// 消費税率の定義
const TAX: f32 = 0.1;

/// 文字列リテラルの定義
const GREETING: &str = "Hello, World!";

/// 定数モジュールの参照
mod constants;
mod locals;

fn main() {
	// 消費税率の参照
	println!("Hello, Tax !{}", TAX);

	// 文字列リテラルの参照
	println!("{}", GREETING);

	// 定数モジュールから定数の参照
	println!("{}", constants::system::KALI);

	// 同ディレクトリ内のモジュールから定数を参照
	println!("{}", locals::X);
}
