/// 
/// 
/// 日本語パス名への対応が未確認です。
/// 
///

use std::path::Path;
extern crate chrono;

fn timestamp() -> String {
	let date = chrono::Local::now();
	// return format!("{}", date.format("%Y%m%d-%H%M%S%.3f"));
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

/// ファイルごとに呼びだされるハンドラーです。
fn on_entry(source_path: &Path, destination_path: &Path) -> std::io::Result<()> {

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
		if dir_name == "dist" {
			return Ok(());
		}
		if dir_name == ".nuxt" {
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
	if args.len() < 2 {
		println!("path?");
		return;
	}
	let path_source = Path::new(&args[1]);
	let path_destination = format!("{}-{}", &args[1], timestamp());
	println!("[TRACE] source: {}", path_source.to_str().unwrap());
	println!("[TRACE] destination: {}", path_destination.as_str());
	let _ = xcopy(path_source, Path::new(path_destination.as_str()), &on_entry);
}
