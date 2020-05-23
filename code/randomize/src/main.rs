extern crate uuid;

use std::ffi::OsStr;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

/// 標準入力から一行の入力を得ます。
fn input_text() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	if ret.unwrap() == 0 {
		return String::new();
	}
	return (*line.trim()).to_string();
}

/// プロンプトを表示し、YES/NO の応答を読み取ります。
fn confirm(message: &str) -> bool {
	println!("{}", message);
	print!("(y/N)> ");
	std::io::stdout().flush().unwrap();
	let line = input_text().to_uppercase();
	if line == "Y" {
		return true;
	}
	if line == "YES" {
		return true;
	}
	return false;
}

/// 新しい名前を生成します。
fn generate_new_name() -> String {
	let uuid = Uuid::new_v4();
	return uuid.hyphenated().to_string();
}

/// ファイルハンドラー
fn on_file_found(e: &Path) -> Result<()> {
	// ディレクトリの名前
	let parent = match e.parent() {
		Some(d) => d,
		None => Path::new(""),
	};

	// 名前
	let name = generate_new_name();

	// 拡張子
	let ext = match e.extension() {
		Some(s) => s,
		None => OsStr::new(""),
	};

	// 新しい名前
	let new_path = parent.join(&name).with_extension(ext);
	println!("{}", new_path.as_os_str().to_str().unwrap());

	// 名前を変更
	std::fs::rename(e, new_path)?;

	return Ok(());
}

fn enumerate(e: &Path, file_handler: &dyn Fn(&Path) -> Result<()>) -> Result<()> {
	if !e.exists() {
		println!("[TRACE] invalid path {}", e.to_str().unwrap());
		return Ok(());
	} else if e.is_dir() {
		let it = std::fs::read_dir(e)?;
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			enumerate(&path, file_handler)?;
		}
		return Ok(());
	} else {
		return file_handler(e);
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}

	for e in &args[1..] {
		let message = format!(
			"{} をランダムな名前に変更します。この操作は元に戻すことができませんがよろしいですか？",
			e
		);
		if !confirm(&message.to_string()) {
			println!("キャンセルされました。");
			continue;
		}
		let path = Path::new(e);
		let _ = enumerate(path, &on_file_found);
	}
}
