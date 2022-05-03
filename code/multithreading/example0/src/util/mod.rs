/// タイムスタンプを生成します。
pub fn get_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}
