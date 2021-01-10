/// 一行の入力を得ます。
fn input_line() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	return line.trim().to_string();
}

/// 文字列を数値に変換します。
fn parse_int(s: &str) -> i64 {
	return match s.trim().parse::<i64>() {
		Ok(n) => n,
		Err(_) => 0,
	};
}

/// 文字列を数値に変換します。
fn parse_float(s: &str) -> f64 {
	return match s.trim().parse::<f64>() {
		Ok(n) => n,
		Err(_) => 0.0,
	};
}

/// エントリーポイント
fn main() {
	loop {
		use std::io::Write;
		print!("> ");
		std::io::stdout().flush().unwrap();
		let line = input_line();
		if line == "" {
			break;
		}
		if line == ":q" || line == ":Q" {
			break;
		}

		if line.find(".").is_some() {
			// 整数
			let digit = parse_float(&line);
			println!("{}", digit);
		} else {
			// 小数
			let digit = parse_int(&line);
			println!("{}", digit);
		}
	}
}
