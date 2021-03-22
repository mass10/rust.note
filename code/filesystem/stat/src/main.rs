extern crate chrono;

/// std::time::SystemTime の文字列表現を返します。
fn format_filetime(time: &std::time::SystemTime) -> String {
	let timestamp = chrono::DateTime::<chrono::Local>::from(*time);
	return format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S%.3f"));
}

fn dump_file_attributes(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let left = std::fs::metadata(std::path::Path::new(path))?;
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

/// ディレクトリをコピーします。
/// # Arguments
/// * `path` パス
/// * `handler` ファイルハンドラー
fn find_file(path: &str, handler: &dyn Fn(&str) -> std::result::Result<i32, Box<dyn std::error::Error>>) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(0);
	}
	if source_path.is_dir() {
		// ディレクトリ内エントリーを走査
		let mut affected = 0;
		for e in std::fs::read_dir(source_path)? {
			let entry = e?;
			let path = entry.path();
			affected = affected + find_file(&path.to_str().unwrap(), handler)?;
		}
		return Ok(affected);
	}
	if source_path.is_file() {
		return handler(path);
	}
	println!("[WARN] 不明なファイルです。[{}]", path);
	return Ok(0);
}

/// ファイル、またはディレクトリを診断します。
///
/// # Arguments
/// * `source_path` パス
pub fn stat(source_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	find_file(source_path, &file_handler)?;
}

/// アプリケーションのエントリーポイントです。
fn main() {
	// コマンドライン引数(自分自身を除く)
	let args: Vec<String> = std::env::args().skip(1).collect();

	for e in args {
		// ファイル、またはディレクトリを診断
		let result = application::stat(&e);
		if result.is_err() {
			println!("[ERROR] <main()> {}", result.err().unwrap());
		}
	}
}
