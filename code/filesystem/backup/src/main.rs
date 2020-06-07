mod stopwatch;
mod util;

///
///
/// 日本語パス名への対応が未確認です。
///
///
use std::path::Path;
extern crate chrono;

/// ファイルごとに呼びだされるハンドラーです。
fn on_entry(source_path: &Path, destination_path: &Path) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	// ファイルをコピー
	std::fs::copy(source_path, destination_path)?;
	println!("{}", destination_path.to_str().unwrap());

	// 待機
	std::thread::sleep(std::time::Duration::from_millis(1));

	return Ok(1);
}

/// ディレクトリをコピーします。
fn find_files(source_path: &Path, destination_path: &Path, handler: &dyn Fn(&Path, &Path) -> std::result::Result<u32, Box<dyn std::error::Error>>) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	// パスの検証
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(0);
	}

	// ディレクトリ
	if source_path.is_dir() {
		let dir_name = source_path.file_name().unwrap();
		if dir_name == "node_modules" {
			return Ok(0);
		}
		if dir_name == ".git" {
			return Ok(0);
		}
		if dir_name == "dist" {
			return Ok(0);
		}
		if dir_name == ".nuxt" {
			return Ok(0);
		}
		if dir_name == "Debug" {
			return Ok(0);
		}
		if dir_name == "Release" {
			return Ok(0);
		}

		// コピー先にディレクトリを作成します。
		std::fs::create_dir_all(destination_path)?;

		// ディレクトリ内エントリーを走査
		let mut files_copied = 0;
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			let path = entry.path();
			files_copied += find_files(&path, destination_path.join(name).as_path(), handler)?;
		}

		return Ok(files_copied);
	}

	// ファイル
	if source_path.is_file() {
		return handler(source_path, destination_path);
	}

	return Ok(0);
}

fn xcopy(source_path: &Path, destination_path: &Path) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	return find_files(source_path, destination_path, &on_entry);
}

fn sub(path: &str, current_timestamp: &String) -> bool {
	// パスを検証
	let path_source = Path::new(&path);
	if !path_source.exists() {
		println!("[ERROR] [{}] はみつかりません。", path_source.to_str().unwrap());
		return false;
	}

	// 新しいパス
	let path_destination = format!("{}-{}", &path, current_timestamp);

	println!("{} [TRACE] destination: {}", util::Util::timestamp0(), path_destination.as_str());

	// ディレクトリ全体を複製します。
	let result = xcopy(path_source, Path::new(path_destination.as_str()));
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return false;
	}

	// コピーされたファイル数
	let files_copied = result.ok().unwrap();

	println!("{} [TRACE] {}個のファイルをコピーしました。", util::Util::timestamp0(), files_copied);

	return true;
}

/// エントリーポイントです。
fn main() {
	// コマンドライン引数(コマンド自身を除く)
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("パスを指定します。");
		return;
	}

	// 処理時間計測用ストップウォッチ
	let stopwatch = stopwatch::Stopwatch::new();

	// タイムスタンプ
	let current_timestamp = util::Util::timestamp1();

	// 複製
	let mut affected = 0;
	for e in args {
		if !sub(&e, &current_timestamp) {
			continue;
		}
		affected += 1;
	}
	if affected == 0 {
		return;
	}

	// サマリー
	println!("{} [TRACE] end. ({})", util::Util::timestamp0(), stopwatch);
}
