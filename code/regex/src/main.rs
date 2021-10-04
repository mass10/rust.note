extern crate regex;

fn red<T: std::fmt::Display>(s: T) -> String {
	return format!("\x1b[31m{}\x1b[39m", s);
}

fn green<T: std::fmt::Display>(s: T) -> String {
	return format!("\x1b[32m{}\x1b[39m", s);
}

/// 正規表現にマッチする文字列があるかどうか調べます。
///
/// ※正しくない
///
/// # Arguments
/// * `regex` - 正規表現
/// * `text` - 文字列
#[allow(unused)]
fn matches(regex: &str, text: &str) -> bool {
	let reg = regex::Regex::new(regex);
	if reg.is_err() {
		panic!("[ERROR] 正規表現がエラー (理由: {})", reg.err().unwrap());
	}

	// find では単純マッチを判定できません。
	let result = reg.unwrap().find(text);
	if result.is_none() {
		return false;
	}
	return true;
}

fn is_match(exp: &str, text: &str) -> bool {
	return regex::Regex::new(exp).unwrap().is_match(text);
}

/// 郵便番号の検査(妥当な用法)
fn is_postcode(unknown: &str) -> bool {
	return is_match("^[0-9][0-9][0-9]-[0-9][0-9][0-9][0-9]$", unknown);
}

fn is_env_line(s: &str) -> bool {
	return is_match("^ENV\\{[a-zA-Z0-9]+\\}$", s);
}

/// 数字のみ
fn is_digit_all(unknown: &str) -> bool {
	return is_match("^[0-9]+$", unknown);
}

/// 郵便番号の検査(ダメな用法)
fn is_postcode_bad(unknown: &str) -> bool {
	return is_match("[0-9][0-9][0-9]-[0-9][0-9][0-9][0-9]", unknown);
}

fn text_status(b: bool) -> String {
	return match b {
		true => green(true),
		false => red(false),
	};
}

fn validate_postcode(unknown: &str) {
	let result = is_postcode(unknown);
	println!("[TRACE] [{:?}] is postcode? -> [{}]", unknown, text_status(result));
}

fn validate_postcode_bad(unknown: &str) {
	let result = is_postcode_bad(unknown);
	println!("[TRACE] [{:?}] is postcode? -> [{}]", unknown, text_status(result));
}

/// 文字列から文字列を抜き出す
///
/// # Arguments
/// * `mail_body` - 対象文字列
///
/// # Returns
/// * 抜き出した文字列
fn retrieve_from(mail_body: &str) -> (String, String) {
	let reg = regex::Regex::new("From: (.+) <(.+)>");
	if reg.is_err() {
		panic!("[ERROR] 正規表現がエラー (理由: {})", reg.err().unwrap());
	}
	for it in reg.unwrap().captures_iter(mail_body) {
		if 2 < it.len() {
			return (String::from(&it[1]), String::from(&it[2]));
		}
	}
	return ("".to_string(), "".to_string());
}

/// フォームの検証
fn validate_from() {
	let mail_body = "From: Billy Preston <billy.preston@gmail.com>";
	let (left, right) = retrieve_from(mail_body);
	println!("[TRACE] From: [{}] [{}]", left, right);
}

/// `path` が Windows 上で存在するかどうかを検証する
fn is_windows_path(path: &str) -> bool {
	if is_match("^[A-Z]:$", path) {
		return true;
	}
	if regex::Regex::new(r"^[A-Z]:\\[a-zA-Z0-9]*").unwrap().is_match(path) {
		return true;
	}
	return false;
}

fn validate_current_path(path: &str) {
	let result = is_windows_path(path);
	println!("[TRACE] [{}] is windows path? -> [{}]", path, text_status(result));
}

fn validate_digit_string(unknown: &str) {
	let result = is_digit_all(unknown);
	println!("[TRACE] [{}] is digit string? -> [{}]", unknown, text_status(result));
}

fn detect_env_varname(s: &str) -> String {
	let reg = regex::Regex::new("^ENV\\{([a-zA-Z0-9]+)\\}$").unwrap();
	for it in reg.captures_iter(s) {
		if 1 < it.len() {
			return String::from(&it[1]);
		}
	}
	return "".to_string();
}

fn validate_env_line(s: &str) {
	let result = is_env_line(s);
	let varname = detect_env_varname(s);
	println!("[TRACE] [{}] is env line? -> [{}] ({})", s, text_status(result), varname);
}

/// エントリーポイント
fn main() {
	//
	// 郵便番号の検査
	//
	{
		println!("[TRACE] $$$ 郵便番号の検査 $$$");
		validate_postcode("");
		validate_postcode("0");
		validate_postcode("00");
		validate_postcode("000");
		validate_postcode("000-");
		validate_postcode("000-0");
		validate_postcode("000-00");
		validate_postcode("000-000");
		validate_postcode("000-0000");
		validate_postcode("000-0000-0000");
		validate_postcode("000-0000\n000-0000");
		println!();
	}

	//
	// 誤った正規表現
	//
	{
		println!("[TRACE] $$$ 郵便番号の検査(誤った用法) $$$");
		validate_postcode_bad("");
		validate_postcode_bad("0");
		validate_postcode_bad("00");
		validate_postcode_bad("000");
		validate_postcode_bad("000-");
		validate_postcode_bad("000-0");
		validate_postcode_bad("000-00");
		validate_postcode_bad("000-000");
		validate_postcode_bad("000-0000");
		validate_postcode_bad("000-0000-0000");
		validate_postcode_bad("000-0000\n000-0000");

		println!();
	}

	//
	// 正規表現で文字列を抜き出す
	//
	{
		validate_from();

		println!();
	}

	//
	// 数字のみ検査
	//
	{
		println!("--- 数字のみ ---");

		validate_digit_string("");
		validate_digit_string("0");
		validate_digit_string("00");
		validate_digit_string("09283127479326548924");
		validate_digit_string("aA");
		validate_digit_string("7ewrhbcejh2346xv1");

		println!();
	}

	//
	// 現在のパスを確認
	//
	{
		println!("--- Windows のパス検証 ---");

		validate_current_path("C:");
		validate_current_path("C:\\");
		validate_current_path("H:\\");
		validate_current_path("Z");
		validate_current_path("Z:\\Windows");
		validate_current_path("C:\\Windows");
		validate_current_path(r"C:\Windows");
		validate_current_path(r"C:\");
		validate_current_path(r"C:\a");
		validate_current_path(r"C:\ab");
		validate_current_path(r"C:\abc");
		validate_current_path(r"C:\ABCD");
		validate_current_path("Z:\\Users");
		validate_current_path("C:/Users");

		println!();
	}

	{
		validate_env_line("");
		validate_env_line("a");
		validate_env_line("ENV{FIELD01}");
	}
}
