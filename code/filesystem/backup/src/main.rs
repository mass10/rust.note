///
///
/// 日本語パス名への対応が未確認です。
///
///
use std::path::Path;
extern crate chrono;

fn timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

/// ファイルごとに呼びだされるハンドラーです。
fn on_entry(source_path: &Path, destination_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
	std::fs::copy(source_path, destination_path)?;
	println!("{}", destination_path.to_str().unwrap());
	std::thread::sleep(std::time::Duration::from_millis(1));
	return Ok(());
}

/// ディレクトリをコピーします。
fn find_files(
	source_path: &Path,
	destination_path: &Path,
	handler: &dyn Fn(&Path, &Path) -> std::result::Result<(), Box<dyn std::error::Error>>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
		if dir_name == "Debug" {
			return Ok(());
		}
		if dir_name == "Release" {
			return Ok(());
		}
		// コピー先にディレクトリを作成します。
		std::fs::create_dir_all(destination_path)?;
		// ディレクトリ内エントリーを走査
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			let path = entry.path();
			let _ = find_files(&path, destination_path.join(name).as_path(), handler);
		}
		return Ok(());
	}
	if source_path.is_file() {
		return handler(source_path, destination_path);
	}
	return Ok(());
}

fn xcopy(source_path: &Path, destination_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
	return find_files(source_path, destination_path, &on_entry);
}

/// エントリーポイントです。
fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}
	let path_source = Path::new(&args[1]);
	if !path_source.exists() {
		println!("[{}] はみつかりません。", path_source.to_str().unwrap());
		return;
	}
	let path_destination = format!("{}-{}", &args[1], timestamp());
	println!("[TRACE] destination: {}", path_destination.as_str());
	let result = xcopy(path_source, Path::new(path_destination.as_str()));
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
