//!
//! 様々なシステム日時を取得するサンプル
//!

extern crate chrono;

fn get_current_timestamp0() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

fn get_current_timestamp1() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

/// 現在のタイムスタンプを取得する(書式付き)
fn get_current_timestamp_with_timezone() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%+"));
}

/// 指定されたタイムゾーンで現在のタイムスタンプを返します。
fn generate_timestamnp_in(hours: i64) -> String {
    let now = chrono::Utc::now();
    let local_time = now + chrono::Duration::hours(hours);
	// +#03 -> 符号付き、最低3桁を確保する、0埋め、整数
	let text = format!("{}{:+#03}:00", local_time.format("%Y-%m-%dT%H:%M:%S%.3f"), hours);
    return text;
}

/// 任意のソーンで現在のタイムスタンプを取得します。
fn get_current_timestamp_with_timezone2() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f %Z"));
}

/// エントリーポイント
fn main() {
	println!("# std::time::SystemTime");
	println!("{:?}", std::time::SystemTime::now());
	println!();

	println!("# timestamp with timezone (chrono::Local)");
	println!("{}", get_current_timestamp_with_timezone());
	println!("{}", get_current_timestamp_with_timezone2());
	println!();

	for i in -12..12 {
		println!("{}", generate_timestamnp_in(i));
	}
	println!();

	println!("# local timestamp");
	println!("{}", get_current_timestamp0());
	println!("{}", get_current_timestamp1());
}
