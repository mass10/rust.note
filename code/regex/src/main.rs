extern crate regex;

fn red<T: std::fmt::Display>(s: T) -> String {
	return format!("\x1b[31m{}\x1b[39m", s);
}

fn green<T: std::fmt::Display>(s: T) -> String {
	return format!("\x1b[32m{}\x1b[39m", s);
}

fn matches(regex: &str, text: &str) -> bool {
	let reg = regex::Regex::new(regex);
	if reg.is_err() {
		panic!("[ERROR] 正規表現がエラー (理由: {})", reg.err().unwrap());
	}
	let result = reg.unwrap().find(text);
	if result.is_none() {
		return false;
	}
	return true;
}

fn is_postcode(unknown: &str) -> bool {
	return matches("^[0-9][0-9][0-9]-[0-9][0-9][0-9][0-9]$", unknown);
}

fn is_postcode_bad(unknown: &str) -> bool {
	return matches("[0-9][0-9][0-9]-[0-9][0-9][0-9][0-9]", unknown);
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

fn validate_from() {
	let mail_body = "From: Billy Preston <billy.preston@gmail.com>";
	let (left, right) = retrieve_from(mail_body);
	println!("[TRACE] From: [{}] [{}]", left, right);
}

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
	}
}
