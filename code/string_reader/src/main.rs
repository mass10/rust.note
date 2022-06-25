/// ファイル全体を読み込みます。
fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

/// イテレータを使って一行ずつ読み込むサンプル
fn case_read_string_into_lines_with_iterator() {
	use std::io::{BufRead, BufReader};

	// サンプル文字列をファイルから読み込み
	let source = read_text_file("Cargo.lock").unwrap();

	// stringreader::StringReader (5年くらい更新なし)
	let reader = stringreader::StringReader::new(&source);
	let bufreader = BufReader::new(reader);

	// イテレータを経由して一行ずつ読み込み
	for e in bufreader.lines() {
		let line = e.unwrap();
		println!("> [{}]", line);
	}
}

/// 一行ずつ読み込むサンプル
fn case_read_string_into_lines_1_by_1() {
	use std::io::{BufRead, BufReader};

	// サンプル文字列をファイルから読み込み
	let source = read_text_file("Cargo.lock").unwrap();

	// stringreader::StringReader (5年くらい更新なし)
	let reader = stringreader::StringReader::new(&source);
	let mut bufreader = BufReader::new(reader);

	loop {
		let mut line = String::new();
		let read = bufreader.read_line(&mut line).unwrap();
		if read == 0 {
			// 終了の条件
			println!("> (EOF)");
			break;
		}
		// 改行が入ってる🔥 (普通そうだっけ？)
		println!("> [{}]", line);
	}
}

/// エントリーポイントです。
fn main() {
	// stringreader::StringReader 版。イテレータ実装
	case_read_string_into_lines_with_iterator();

	// stringreader::StringReader 版。read_line による読み込み。
	case_read_string_into_lines_1_by_1();
}
