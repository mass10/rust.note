use std::io::Read;

// use std::{net::ToSocketAddrs, ops::Add};

/// ディレクトリまたはファイルを削除します。
fn unlink(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let e = std::path::Path::new(path);
	if !e.exists() {
		return Ok(());
	}
	if e.is_dir() {
		println!("[TRACE] {} を削除中...", path);
		std::fs::remove_dir_all(path)?;
		return Ok(());
	}
	if e.is_file() {
		println!("[TRACE] {} を削除中...", path);
		std::fs::remove_file(path)?;
		return Ok(());
	}
	return Ok(());
}

struct MyArchiver {
	// archiver: Option<zip::ZipWriter<std::fs::File>>,
}

impl MyArchiver {
	/// 作成
	pub fn new() -> MyArchiver {
		return MyArchiver {};
	}

	fn append_entry(&mut self, archiver: &mut zip::ZipWriter<std::fs::File>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		#[allow(unused)]
		use std::io::Write;

		let unknown = std::path::Path::new(path);
		if !unknown.exists() {
			return Ok(());
		} else if unknown.is_dir() {
			println!("[TRACE] サブディレクトリ {} を追加中...", path);

			let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
			archiver.add_directory(path, options)?;
			let reader = unknown.read_dir()?;
			for e in reader {
				let entry = e.unwrap();
				self.append_entry(archiver, entry.path().to_str().unwrap())?;
			}
		} else if unknown.is_file() {
			println!("[TRACE] ファイル {} を追加中...", path);

			// 名前
			let name = unknown.file_name().unwrap().to_str().unwrap().to_string();
			// zip 内におけるルートからの絶対パス "/a/b/c/d.txt"
			let _relative_path = unknown.join(name).to_str().unwrap().to_string();
			let relative_path = unknown.to_str().unwrap();
			println!("[TRACE] >> ファイル {}", relative_path);

			let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
			archiver.start_file(relative_path, options)?;

			let mut stream = std::fs::File::open(relative_path)?;
			loop {
				let mut buffer = [0; 5];
				let bytes_read = stream.read(&mut buffer)?;
				if bytes_read == 0 {
					break;
				}
				let write_buffer = &buffer[..bytes_read];
				let bytes_written = archiver.write(&write_buffer)?;
				println!("[TRACE] (read {} bytes, written {} bytres...)", bytes_read, bytes_written);
			}
			// archiver.write(b"Hello, World!")?;
			// archiver.finish()?;
		}
		return Ok(());
	}

	fn create_zip_name(path: &str) -> String {
		let pathname = path.to_string() + ".zip";
		return pathname;
	}

	/// ディレクトリをアーカイブします。
	pub fn archive(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		println!("[TRACE] {} をアーカイブ中...", path);
		// use std::io::Write;

		// ファイル名を生成
		let archive_path = MyArchiver::create_zip_name(path);

		// .zip ファイルがあれば削除
		unlink(&archive_path)?;

		// アーカイバーの初期化
		let w = std::fs::File::create(archive_path)?;
		let mut archiver = zip::ZipWriter::new(w);

		// ここから走査
		self.append_entry(&mut archiver, path)?;

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

	let path = &args[0];

	// unlink("src.zip")?;

	// zip
	let mut ar = MyArchiver::new();
	let result = ar.archive(&path);
	if result.is_err() {
		let error = result.err();
		println!("[ERROR] {}", error.unwrap());
		return;
	}

	println!("Ok.");
}
