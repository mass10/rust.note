mod stopwatch;
mod util;

extern crate chrono;

/// ディレクトリをコピーします。
fn find_files(left: &str, out: &TextFileWriter) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(left);
	// パスの検証
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", left);
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
		// std::fs::create_dir_all(destination_path)?;

		// ディレクトリ内エントリーを走査
		let mut files_copied = 0;
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			// let name = entry.file_name();
			let path = entry.path();
			files_copied += find_files(path.to_str().unwrap(), out)?;
		}

		return Ok(files_copied);
	}

	// ファイル
	if source_path.is_file() {
		out.append_line(left)?;
		return Ok(1);
	}

	return Ok(0);
}

// type Handler = dyn Fn(String) -> std::result::Result<(), Box<dyn std::error::Error>>;

fn collect(source_path: &str, list_file: &str) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	let writer = TextFileWriter::new(list_file)?;
	return find_files(source_path, &writer);
}

use std::io::Write;

struct TextFileWriter {
	_file: Option<Box<std::fs::File>>,
}

fn _test_write(out: &mut std::fs::File) -> std::result::Result<(), Box<dyn std::error::Error>> {
	out.write_all(b"")?;
	return Ok(());
}

// fn _open(path: &str) -> std::result::Result<TextFileWriter, Box<dyn std::error::Error>>
// 	return std::fs::File::create(path)?,
// 	let file = match std::path::Path::new(path).exists() {
// 		true => std::fs::File::create(path)?,
// 		false => std::fs::File::create(path)?,
// 	}
// }

impl TextFileWriter {
	pub fn new(path: &str) -> std::result::Result<TextFileWriter, Box<dyn std::error::Error>> {
		println!("[TRACE] ファイルを作成しています... {}", path);
		if std::path::Path::new(path).exists() {
			std::fs::remove_file(path)?;
		}
		let file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
		let file = Box::new(file);
		let instance = TextFileWriter { _file: Some(file) };
		return Ok(instance);
	}

	pub fn _append_line_bak(self: &TextFileWriter, line: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let file = &self._file;
		let out = file.as_ref().unwrap();
		let mut f = out.as_ref();
		f.write_all(line.as_bytes())?;
		return Ok(());
	}

	pub fn append_line(self: &TextFileWriter, line: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		println!("[TRACE] 追記 {}", line);
		let file = &self._file;
		let out = file.as_ref().unwrap();
		let mut f = out.as_ref();
		f.write_all(line.as_bytes())?;
		f.write_all(b"\n")?;
		return Ok(());
	}
}

fn call_zip7(left: &str, right: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// 7zip 呼び出し
	let command_path = "C:\\Program Files\\7-Zip\\7z.exe";
	let mut command = std::process::Command::new(command_path);
	let archive_name = right;
	let list_name = format!("@{}", left);
	let args = ["a", archive_name, list_name.as_str()];
	let mut command = command.args(&args).spawn()?;
	let response = command.wait();
	if response.is_err() {
		return Err(Box::new(response.err().unwrap()));
	}

	// 終了ステータスを確認
	let status = response.unwrap();
	if !status.success() {
		// バッチを終了
		let exit_code = status.code().unwrap();
		println!("[WARN] yarn exited with status: {}", exit_code);
		std::process::exit(exit_code);
	}

	return Ok(());
}

fn zip_main(path: &str) -> bool {
	// タイムスタンプ
	let current_timestamp = util::Util::timestamp1();

	// 新しいパス
	let archive_name = format!("{}-{}.zip", path, current_timestamp);
	println!("{} [TRACE] destination: {}", util::Util::timestamp0(), archive_name.as_str());

	// リストファイル名
	let listfile = "~listfile.tmp";

	// バックアップ対象ファイルを列挙します。
	let result = collect(path, listfile);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return false;
	}

	// 書庫化
	let result2 = call_zip7(listfile, archive_name.as_str());
	if result2.is_err() {
		println!("[ERROR] 書庫化に失敗しています。理由: {}", result2.err().unwrap());
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

	let target = &args[0];

	// 処理時間計測用ストップウォッチ
	let sw = stopwatch::Stopwatch::new();

	// 複製
	if !zip_main(&target) {
		return;
	}

	// 処理時間
	let duration = sw.elapsed();

	// サマリー
	println!("{} [TRACE] end. ({})", util::Util::timestamp0(), duration);
}
