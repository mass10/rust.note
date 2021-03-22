use std::io::Read;

use chrono::{Datelike, Timelike};

// use std::{net::ToSocketAddrs, ops::Add};

#[allow(unused)]
fn get_file_name(path: &str) -> String {
	let unknown = std::path::Path::new(path);
	return unknown.file_name().unwrap().to_str().unwrap().to_string();
}

/// パスを正しい形式に変換します。
fn canonicalize_path(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let path = std::path::Path::new(path);
	let path = path.canonicalize()?;
	let path = path.to_str().unwrap().to_string();
	return Ok(path);
}

/// ディレクトリまたはファイルを削除します。
fn unlink(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	if path == "" {
		return Ok(());
	}
	if path == "/" {
		return Ok(());
	}
	if path == "C:" {
		return Ok(());
	}
	if path == "C:\\" {
		return Ok(());
	}
	if path == "C:\\Windows" {
		return Ok(());
	}

	let e = std::path::Path::new(path);
	if !e.exists() {
		return Ok(());
	}

	if e.is_dir() {
		std::fs::remove_dir_all(path)?;
		return Ok(());
	}
	if e.is_file() {
		std::fs::remove_file(path)?;
		return Ok(());
	}
	return Ok(());
}

struct MyArchiver {
	// archiver: Option<zip::ZipWriter<std::fs::File>>,
}

fn build_path(base_name: &str, name: &str) -> String {
	let unknown = std::path::Path::new(base_name);
	let path_name = unknown.join(name);
	return path_name.to_str().unwrap().to_string();
}

impl MyArchiver {
	/// 作成
	pub fn new() -> MyArchiver {
		return MyArchiver {};
	}

	fn append_entry(&mut self, archiver: &mut zip::ZipWriter<std::fs::File>, base_name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		use std::io::Write;

		let unknown = std::path::Path::new(path);
		if !unknown.exists() {
			return Ok(());
		} else if unknown.is_dir() {
			let name = unknown.file_name().unwrap().to_str().unwrap();

			// 内部ディレクトリエントリーを作成
			if base_name != "" {
				let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
				archiver.add_directory(build_path(base_name, name), options)?;
			}

			let it = std::fs::read_dir(path)?;
			for e in it {
				let entry = e?;

				// base_name からの内部相対パスを生成
				let sub_dir_name = build_path(base_name, &name);

				let path_name = entry.path();
				let path_name = path_name.to_str().unwrap();

				self.append_entry(archiver, &sub_dir_name, path_name)?;
			}
		} else if unknown.is_file() {
			let full_path = unknown.to_str().unwrap();
			let name = unknown.file_name().unwrap().to_str().unwrap();

			let relative_path = build_path(base_name, name);

			println!("adding file ... {}", &relative_path);

			let meta = unknown.metadata()?;
			let last_modified = meta.modified()?;
			let chrono_timestamp = chrono::DateTime::<chrono::Local>::from(last_modified);

			// ファイルをアーカイブ
			let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

			// 最終更新日時
			let last_modified_time = zip::DateTime::from_date_and_time(
				chrono_timestamp.year() as u16,
				chrono_timestamp.month() as u8,
				chrono_timestamp.day() as u8,
				chrono_timestamp.hour() as u8,
				chrono_timestamp.minute() as u8,
				chrono_timestamp.second() as u8,
			)
			.unwrap();
			let options = options.last_modified_time(last_modified_time);

			// let options = options.last_modified_time(last_modified);
			archiver.start_file(&relative_path, options)?;

			let mut stream = std::fs::File::open(full_path)?;
			loop {
				let mut buffer = [0; 1000];
				let bytes_read = stream.read(&mut buffer)?;
				if bytes_read == 0 {
					break;
				}
				let write_buffer = &buffer[..bytes_read];
				archiver.write(&write_buffer)?;
			}
			// archiver.write(b"Hello, World!")?;
			// archiver.finish()?;
		}

		return Ok(());
	}

	/// ディレクトリをアーカイブします。
	pub fn archive(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {

		// パスを正規化
		let path = canonicalize_path(path)?;

		println!("archiving ... {}", &path);

		// ファイル名を生成
		let archive_path_name = path.to_string() + ".zip";

		println!("[TRACE] [{}]", archive_path_name);

		// 起点となるディレクトリの名前
		let base_name = "";

		// .zip ファイルがあれば削除
		unlink(&archive_path_name)?;

		// アーカイバーの初期化
		let w = std::fs::File::create(archive_path_name)?;
		let mut archiver = zip::ZipWriter::new(w);

		// ここから走査
		self.append_entry(&mut archiver, &base_name, &path)?;

		// アーカイバーを閉じます。
		archiver.finish()?;

		return Ok(());
	}
}

/// 使用方法を表示します。
fn usage() {
	println!("[ERROR] ディレクトリ名を指定します。");
}

/// アプリケーションのエントリーポイント
fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		usage();
		return;
	}

	for path in &args {
		// それぞれを、それぞれの zip にアーカイブします。
		let mut ar = MyArchiver::new();
		let result = ar.archive(&path);
		if result.is_err() {
			let error = result.err();
			println!("[ERROR] {}", error.unwrap());
			return;
		}
	}

	println!("Ok.");
}
