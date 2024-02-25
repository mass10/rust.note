/// ユーティリティトレイト
trait AppStringUtil {
	/// この文字列が BOM で開始しているかどうかを返す
	fn has_bom(&self) -> bool;
}

impl AppStringUtil for &str {
	fn has_bom(&self) -> bool {
		if self.len() < 3 {
			return false;
		}

		if self.as_bytes()[0] == 0xEF && self.as_bytes()[1] == 0xBB && self.as_bytes()[2] == 0xBF {
			return true;
		}

		return false;
	}
}

/// ファイル全体の内容を String で返します。
#[allow(unused)]
fn read_text_file_string(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content)?;

	return Ok(content);
}

/// ファイル全体の内容を Vec<u8> で返します。
fn read_text_file_vec_u8(path: &str) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut content: Vec<u8> = vec![];
	file.read_to_end(&mut content)?;

	return Ok(content);
}

fn u8_to_u16(buffer: &[u8]) -> Vec<u16> {
	let mut result: Vec<u16> = vec![];

	for i in 0..buffer.len() / 2 {
		let b1 = buffer[i * 2] as u16;
		let b2 = buffer[i * 2 + 1] as u16;
		let s = b1 | (b2 << 8);
		result.push(s);
	}

	return result;
}

/// UTF-16 LE テキストファイルの全体を読み込みます。
fn read_text_file_utf16(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let buffer: Vec<u8> = read_text_file_vec_u8(path)?;

	if buffer.len() < 2 {
		return Err("Invalid file size".into());
	}

	if buffer[0] != 0xFF || buffer[1] != 0xFE {
		return Err("Invalid BOM".into());
	}

	let buffer = &buffer[2..];

	let shorts = u8_to_u16(&buffer);

	let result = String::from_utf16(&shorts)?;

	return Ok(result);
}

///
fn u16_to_u8_vec(buffer: &[u16]) -> Vec<u8> {
	let mut result: Vec<u8> = vec![];

	for i in 0..buffer.len() {
		let b1 = (buffer[i] & 0x00FF) as u8;
		let b2 = ((buffer[i] & 0xFF00) >> 8) as u8;
		result.push(b1);
		result.push(b2);
	}

	return result;
}

fn create_utf16_file(
	path: &str,
	content: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
	use std::io::Write;

	let mut file = std::fs::File::create(path)?;

	let bytes = content.encode_utf16().collect::<Vec<u16>>();

	let bytes = u16_to_u8_vec(&bytes);

	// BOM
	file.write_all(&[0xFF, 0xFE])?;

	file.write_all(&bytes)?;

	return Ok(());
}

/// Rust アプリケーションのエントリーポイントです。
fn main() {
	let source = "こんにちは お晩です รถตำรวจ\r\n\r\n";

	create_utf16_file("u16.txt", &source).unwrap();

	let content = read_text_file_utf16("u16.txt").unwrap();

	eprintln!("[{}]", content);

	assert!(source == content);
}
