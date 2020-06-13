extern crate chrono;

// use chrono::prelude::*;
// use chrono::Utc;
// use chrono::TimeZone::Utc;

/// アプリケーション本体
mod application {

	#[allow(unused)]
	fn format_filetime(time: std::time::SystemTime) -> String {
		let timestamp: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(time);
		let timestamp: chrono::DateTime<chrono::Local> = chrono::DateTime::from(time);
		let timestamp = chrono::DateTime::<chrono::Local>::from(time);
		if 
		return format!("{}", timestamp);
	}

	#[allow(unused)]
	fn format_filetime2(time: std::time::SystemTime) -> String {
		let timestamp = time.elapsed().unwrap();

		// return format!("{:?}", time);

		let duration = time.duration_since(std::time::SystemTime::UNIX_EPOCH);
		if duration.is_err() {
			return String::new();
		}
		let duration = duration.unwrap();
		let mut secs = duration.as_secs();
		let mut min = 0;
		while (60 <= secs) {
			min += 1;
			secs -= 60;
		}
		let mut hh = 0;
		while (60 <= min) {
			hh += 1;
			min -= 60;
		}
		let mut dd = 0;
		while (24 <= hh) {
			hh -= 24;
			dd += 1;
		}
		let mut mm = 0;
		let mut yyyy = 0;
		while (365 <= dd) {
			dd -= 365;
			yyyy += 1;
		}
		return format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", yyyy, mm, dd, hh, min, secs);
	}

	fn dump_file_attributes(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// 元
		let path = std::path::Path::new(path);
		let left = std::fs::metadata(path)?;
		println!("◆ファイル: {:?}", path);
		println!("    サイズ: {}", left.len());
		let system_time = left.modified()?;
		println!("    タイムスタンプ: {:?}", format_filetime(system_time));
		println!();
		return Ok(());
	}

	/// ファイルごとに呼びだされるハンドラーです。
	fn file_handler(source_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
		dump_file_attributes(source_path)?;
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
}

/// エントリーポイントです。
fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		let result = application::stat(&e);
		if result.is_err() {
			println!("[ERROR] <main()> {}", result.err().unwrap());
		}
	}
}
