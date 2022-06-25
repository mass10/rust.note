fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

/// ファイル全体を文字列として読み込む例
fn test1(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let content = read_text_file(path)?;
	println!("{}", content);
	return Ok(());
}

/// ファイルを1行ずつ読み込む例
fn test2(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	use std::io::BufRead;

	let file = std::fs::File::open(path)?;
	let r = std::io::BufReader::new(file);

	// lines() はイテレーターを返す。全体をメモリにロードするわけではなさそう？
	for e in r.lines() {
		let line = e.unwrap();
		println!("{}", line);
	}

	return Ok(());
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("path to file?");
		return Ok(());
	}

	let path = &args[0];

	// reading whole text file.
	println!("--- TEST 1 ---");
	test1(path)?;

	// reading a text file line-by-line.
	println!("--- TEST 2 ---");
	test2(path)?;

	return Ok(());
}
