use std::io::Write;

/// 現在のタイムスタンプを文字列で返します。
///
/// # Returns
/// タイムスタンプ
pub fn get_current_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// 一度破棄して出力します。
///
/// # Arguments
/// * path パス
/// * line 文字列
fn trunc_and_append_line(path: &str, line: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut f = std::fs::File::create(path)?;
	f.write_all(line.as_bytes())?;
	f.write_all(b"\n")?;
	return Ok(());
}

/// テキストファイルに行を出力します。
///
/// # Arguments
/// * path パス
/// * line 文字列
fn append_line(path: &str, line: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	// create or append
	let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
	f.write_all(line.as_bytes())?;
	f.write_all(b"\n")?;
	return Ok(());
}

/// アプリケーションのエントリーポイントです。
fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let path = "main.log";

	append_line(path, "文字列")?;
	trunc_and_append_line(path, "truncated.")?;
	append_line(path, "aaa")?;
	append_line(path, "bbb")?;
	append_line(path, "ccc")?;

	return Ok(());
}
