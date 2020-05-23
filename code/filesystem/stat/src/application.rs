// fn get_filetime(s: &str) -> std::result::Result<String, std::io::Error> {
// 	let right_attribute = std::fs::metadata(s)?;
// 	let file_time = right_attribute.modified()?;
// 	let timestamp = format!("{:?}", file_time);
// 	return Ok(timestamp);
// }

fn dump_attributes(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// 元
	let path = std::path::Path::new(path);
	let left = std::fs::metadata(path)?;
	println!("◆ファイル: {:?}", path);
	println!("    サイズ: {}", left.len());
	let system_time = left.modified()?;
	println!("    タイムスタンプ: {:?}", system_time);
	println!();
	return Ok(());
}

/// ファイルごとに呼びだされるハンドラーです。
fn file_handler(source_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	dump_attributes(source_path)?;
	std::thread::sleep(std::time::Duration::from_millis(1));
	return Ok(1);
}

/// ディレクトリをコピーします。
fn find_file(path: &str, handler: &dyn Fn(&str) -> std::result::Result<i32, Box<dyn std::error::Error>>) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(0);
	}
	if source_path.is_dir() {
		// ディレクトリ内エントリーを走査
		let it = std::fs::read_dir(source_path)?;
		let mut affected = 0;
		for e in it {
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

/// アプリケーションクラス
pub fn stat(source_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	return find_file(source_path, &file_handler);
}
