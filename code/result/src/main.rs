fn diagnose_request(s: &str) -> std::result::Result<String, String> {
	let s = s.to_string().to_uppercase();
	let s = s.as_str();
	match s {
		"ALL MY LOVING" => Ok(String::from("The Beatles")),
		"SHE'S NOT THERE" => Ok(String::from("The Zombies")),
		"BREAKING THE LAW" => Err(String::from("Judas Priest")),
		"WAR PIGS" => Ok(String::from("Black Sabbath")),
		"PAINKILLER" => Err(String::from("Judas Priest")),
		_ => Err(String::from("You go home, boy!")),
	}
}

fn main() {
	// コマンドライン引数を結合し、リクエストとします。
	let args: Vec<String> = std::env::args().skip(1).collect();
	let request_string = args.join(" ");

	// リクエストを読み取ります。
	let result = diagnose_request(request_string.as_str());
	if result.is_err() {
		let answer = result.err().unwrap();
		println!("[Err] {}", answer);
		return;
	}

	let response = result.unwrap();
	println!("[Ok] {}", response);
}
