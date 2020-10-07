extern crate md5;

fn digest_md5(s: &str) -> String {
	return format!("{:x}", md5::compute(s));
}

fn test(s: &str) {
	let digest = digest_md5(s);
	println!("[TRACE] [{}] >> [{}] (MD5)", s, digest);
}

fn dump_file_md5(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	use std::io::Read;

	// ファイルを開きます。
	let mut file = std::fs::File::open(path)?;

	// 読み取り用バッファ
	let mut buffer = std::vec::Vec::new();

	// ファイル全体を読み込み
	file.read_to_end(&mut buffer)?;

	// MD5 を計算
	let response = md5::compute(buffer);
	println!("{:x}", response);

	return Ok(());
}

#[allow(unused)]
fn main_bak() {
	test("Hello, world!");
	test("コンニチハ！");
	test("https://www.nittsu.co.jp");
	test("https://www.mazda.co.jp");
	test("https://www.mol.co.jp");
	test("https://www.nissan.co.jp");
	test("https://www.marubeni.com");
	test("https://www.hd.eneos.co.jp");
}

/// エントリーポイント
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// コマンドライン引数を先頭から一つだけ取り出します。
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("path?");
		return Ok(());
	}
	let path = &args[0];

	// ファイルの MD5 を出力します。
	dump_file_md5(path)?;

	return Ok(());
}
