extern crate chrono;

mod stopwatch;
mod util;
use std::io::Write;

/// ディレクトリを走査します。
fn find_file(left: &str, out: &mut std::fs::File) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	// 元
	let source_path = std::path::Path::new(left);
	if !source_path.exists() {
		println!("[ERROR] ディレクトリは存在しません。{}", left);
		return Ok(0);
	}

	// ディレクトリ
	if source_path.is_dir() {
		// ディレクトリ内エントリーを走査
		let mut count = 0;
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			if name == ".listfile.tmp" {
				continue;
			}
			let path = entry.path();
			count += find_file(&path.to_str().unwrap(), out)?;
		}
		return Ok(count);
	}

	// ファイル
	if source_path.is_file() {
		println!(">> {}", left);
		let left = match left.starts_with(".\\") {
			true => &left[2..],
			_ => left,
		};
		out.write_all(left.as_bytes())?;
		out.write_all(b"\n")?;
		return Ok(1);
	}

	return Ok(0);
}

/// ディレクトリをコピーします。
fn xcopy(left: &str, right: &str) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	// 元
	let source_path = std::path::Path::new(left);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", left);
		return Ok(0);
	}

	// 先
	let destination_path = std::path::Path::new(right);

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
			files_copied += xcopy(&path.to_str().unwrap(), destination_path.join(name).as_path().to_str().unwrap())?;
		}

		return Ok(files_copied);
	}

	// ファイル
	if source_path.is_file() {
		std::fs::copy(source_path, destination_path)?;
		std::thread::sleep(std::time::Duration::from_millis(1));
		return Ok(1);
	}

	return Ok(0);
}

fn _test_write(out: &mut std::fs::File) -> std::result::Result<(), Box<dyn std::error::Error>> {
	out.write_all(b"")?;
	return Ok(());
}

fn call_zip7(left: &str, right: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// ロケーション移動
	std::env::set_current_dir(&left)?;

	// 7zip 呼び出し
	let command_path = "C:\\Program Files\\7-Zip\\7z.exe";
	let mut command = std::process::Command::new(command_path);
	let archive_name = right;
	let listfile_tmp = ".listfile.tmp";
	let listfile = format!("@{}", listfile_tmp);
	let args = ["a", archive_name, &listfile];
	let mut command = command.args(&args).spawn()?;
	let status = command.wait()?;

	// 終了ステータスを確認
	if !status.success() {
		// バッチを終了
		let exit_code = status.code().unwrap();
		println!("[WARN] yarn exited with status: {}", exit_code);
		std::process::exit(exit_code);
	}

	return Ok(());
}

fn create_listfile(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// ロケーション移動
	std::env::set_current_dir(&path)?;

	// リストファイル名
	let listfile_tmp = ".listfile.tmp";
	let mut out = std::fs::OpenOptions::new().create(true).append(true).open(listfile_tmp)?;

	// リストファイル出力
	find_file(".", &mut out)?;
	out.flush()?;

	return Ok(());
}

fn zip_main(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// タイムスタンプ(%Y%m%d-%H%M%S)
	let current_timestamp = util::Util::timestamp1();

	// 一時ディレクトリ
	let tmp_dir = tmp_dir()?;

	// 新しいパス
	let archive_name = format!("{}-{}.zip", path, current_timestamp);
	println!("{} [TRACE] destination: {}", util::Util::timestamp0(), archive_name.as_str());

	// バックアップ対象ファイルを列挙します。
	let files_copied = xcopy(path, tmp_dir.as_str())?;

	// リストファイルを作成
	create_listfile(&tmp_dir)?;

	// 書庫化
	call_zip7(&tmp_dir, archive_name.as_str())?;
	println!("{} [TRACE] {}個のファイルをコピーしました。", util::Util::timestamp0(), files_copied);

	return Ok(());
}

fn tmp_dir() -> std::result::Result<String, Box<dyn std::error::Error>> {
	// タイムスタンプ(%Y%m%d-%H%M%S)
	let current_timestamp = util::Util::timestamp1();
	let path = format!(".{}.tmp", current_timestamp);
	std::fs::create_dir(&path)?;
	return Ok(path);
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
	let result = zip_main(&target);
	if result.is_err() {
		println!("[ERROR] エラー！理由: {:?}", result.err().unwrap());
		return;
	}

	// 処理時間
	let duration = sw.elapsed();

	// サマリー
	println!("{} [TRACE] end. ({})", util::Util::timestamp0(), duration);
}
