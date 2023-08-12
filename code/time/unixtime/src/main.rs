extern crate chrono;

fn parse_int(s: &str) -> i64 {
	return match s.trim().parse::<i64>() {
		Ok(n) => n,
		Err(_) => 0,
	};
}

/// [chrono::NaiveDateTime] を [std::time::SystemTime] に変換する
fn chrono_time_to_systemtime(time: &chrono::NaiveDateTime) -> std::time::SystemTime {
    let time = chrono::DateTime::<chrono::Utc>::from_utc(*time, chrono::Utc);
	return time.into();
}

fn second_to_systemtime(secs: i64) -> std::time::SystemTime {
	let time = chrono::NaiveDateTime::from_timestamp(secs, 0);
	return chrono_time_to_systemtime(&time);
}

fn format_timestamp_as_local_time_bak(secs: i64) -> String {
	let systime = second_to_systemtime(secs);
    let time = chrono::DateTime::<chrono::Local>::from(systime);
    return format!("{}", time.format("%Y-%m-%d %H:%M:%S.%3f"));
}

fn format_timestamp_as_local_time(secs: i64) -> String {
	let time = chrono::NaiveDateTime::from_timestamp(secs, 0);
	let time = chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc);
    let time = chrono::DateTime::<chrono::Local>::from(time);
    return format!("{}", time.format("%Y-%m-%d %H:%M:%S.%3f"));
}

fn format_timestamp_as_utc_bak(secs: i64) -> String {
	let systime = second_to_systemtime(secs);
	let time = chrono::DateTime::<chrono::Utc>::from(systime);
	return format!("{}", time.format("%Y-%m-%d %H:%M:%S.%3f"));
}

fn format_timestamp_as_utc(secs: i64) -> String {
	let time = chrono::NaiveDateTime::from_timestamp(secs, 0);
	return format!("{}", chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc));
}

/// タイムスタンプの診断
fn diagnose_digit_as_timestamp(secs: i64) {
	let localtime = format_timestamp_as_local_time(secs);
	let utc_time = format_timestamp_as_utc(secs);
	println!("{}: LOCAL {:?}, UTC {:?}", secs, localtime, utc_time);
}

/// ローカルタイムスタンプの表示
fn describe_current_timestamp_local() {
	let now = chrono::Local::now();

	// 数値にしたときは、タイムゾーンが関係無くなります。
	let timestamp = now.timestamp();

	diagnose_digit_as_timestamp(timestamp);
}

/// UTC タイムスタンプの表示
fn describe_current_timestamp_utc() {
	let now = chrono::Utc::now();

	// 数値にしたときは、タイムゾーンが関係無くなります。
	let timestamp = now.timestamp();

	diagnose_digit_as_timestamp(timestamp);
}

/// エントリーポイント
fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		describe_current_timestamp_local();
		describe_current_timestamp_utc();
		return;
	}


	let secs = parse_int(&args[0]);

	diagnose_digit_as_timestamp(secs);
}
