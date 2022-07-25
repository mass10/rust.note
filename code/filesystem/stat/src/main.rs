extern crate chrono;

/// std::time::SystemTime の文字列表現を返します。
fn format_filetime(time: &std::time::SystemTime) -> String {
	let timestamp = chrono::DateTime::<chrono::Local>::from(*time);
	return format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S%.3f"));
}

fn dump_file_attributes(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let path_object = std::path::Path::new(path);
	let left = std::fs::symlink_metadata(path)?;
	// let left = std::fs::metadata(path)?;
	println!("* exists: [{}]", path_object.exists());
	println!("* is_symlink: [{}]", left.is_symlink());
	println!("* is_dir: [{}]", left.is_dir());
	println!("* is_file: [{}]", left.is_file());
	let system_time = left.modified()?;
	println!("{} ({}, {} bytes)", &path, format_filetime(&system_time), left.len());
	return Ok(());
}

/// ファイルごとに呼びだされるハンドラーです。
///
/// # Arguments
/// * `source_path` パス
fn file_handler(source_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	dump_file_attributes(source_path)?;
	// std::thread::sleep(std::time::Duration::from_millis(1));
	return Ok(1);
}

type FileHandler = dyn Fn(&str) -> std::result::Result<i32, Box<dyn std::error::Error>>;

/// ディレクトリをコピーします。
/// # Arguments
/// * `path` パス
/// * `handler` ファイルハンドラー
fn find_file(path: &str, handler: &FileHandler) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(());
	}
	if source_path.is_dir() {
		// ディレクトリ内エントリーを走査
		for e in std::fs::read_dir(source_path)? {
			let entry = e?;
			let path = entry.path();
			find_file(&path.to_str().unwrap(), handler)?;
		}
		return Ok(());
	}
	if source_path.is_file() {
		handler(path)?;
		return Ok(());
	}
	println!("[WARN] 不明なファイルです。[{}]", path);
	return Ok(());
}

/// ファイル、またはディレクトリを診断します。
///
/// # Arguments
/// * `source_path` パス
pub fn stat(source_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	find_file(source_path, &file_handler)?;
	return Ok(());
}

/// アプリケーションのエントリーポイントです。
fn main() {
	// コマンドライン引数(自分自身を除く)
	let args: Vec<String> = std::env::args().skip(1).collect();

	for e in args {
		// ファイル、またはディレクトリを診断
		let result = stat(&e);
		if result.is_err() {
			println!("[ERROR] <main()> {}", result.err().unwrap());
		}
	}
}
