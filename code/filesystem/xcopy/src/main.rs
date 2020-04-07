use std::path::Path;
use file_diff::{diff};

/// プロンプトを表示し、YES/NO の応答を読み取ります。
fn prompt() -> bool {

	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		return false;
	}
	if ret.unwrap() == 0 {
		return false;
	}
	if line.trim().to_uppercase() == "Y" {
		return true;
	}
	if line.trim().to_uppercase() == "YES" {
		return true;
	}
	return false;
}

/// 二つのファイルが同一かどうかを調べます。
fn seems_to_be_same(source_path: &Path, destination_path: &Path) -> bool {

	// 元
	let left = std::fs::metadata(source_path).unwrap();
	// 先
	let right_attribute = std::fs::metadata(destination_path);
	if right_attribute.is_err() {
		// 先のファイルがみつからないようです。
		return false;
	}
	let right = right_attribute.unwrap();
	// サイズとタイムスタンプが同じなら同じとみなします。
	if left.len() == right.len() {
		if left.modified().unwrap() == right.modified().unwrap() {
			return true
		}
	}
	// 中身を比較
	return diff(source_path.to_str().unwrap(), destination_path.to_str().unwrap());
}

/// ファイルごとに呼びだされるハンドラーです。
fn on_entry(source_path: &Path, destination_path: &Path) -> std::io::Result<()> {

	if seems_to_be_same(source_path, destination_path) {
		return Ok(());
	}
	println!("ファイル {} を上書きしますか？(y/N)", destination_path.as_os_str().to_str().unwrap());
	if !prompt() {
		return Ok(());
	}
	let _result = std::fs::copy(source_path, destination_path);
	return Ok(());
}

/// ディレクトリをコピーします。
fn xcopy(source_path: &Path, destination_path: &Path, handler: &dyn Fn(&Path, &Path) -> std::io::Result<()>) -> std::io::Result<()> {

	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(());
	}
	if source_path.is_dir() {
		let dir_name = source_path.file_name().unwrap();
		if dir_name == "node_modules" {
			return Ok(());
		}
		if dir_name == ".git" {
			return Ok(());
		}
		// コピー先にディレクトリを作成します。
		let result = std::fs::create_dir_all(destination_path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] reason: {}", error);
			return Ok(());
		}
		// ディレクトリ内エントリーを走査
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e.unwrap();
			let name = entry.file_name();
			let path = entry.path();
			let _ = xcopy(&path, destination_path.join(name).as_path(), handler);
		}
		return Ok(());
	}
	if source_path.is_file() {
		let file_name = source_path.file_name().unwrap();
		if file_name == ".gitignore" {
			return Ok(())
		}
		if file_name == ".project" {
			return Ok(())
		}
		if file_name.to_os_string().to_str().unwrap().starts_with(".env.") {
			return Ok(())
		}
		return handler(source_path, destination_path);
	}
	return Ok(())
}

/// エントリーポイントです。
fn main() {

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 3 {
		println!("path?");
		return;
	}
	let path_source = Path::new(&args[1]);
	let path_destination = Path::new(&args[2]);
	let _ = xcopy(path_source, path_destination, &on_entry);
}
