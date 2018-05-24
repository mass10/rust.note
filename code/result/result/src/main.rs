fn run(s: &str) -> std::result::Result<String, String> {

	let s = s.to_string().to_uppercase();
	let s = s.as_str();
	match s {
		"OK" => Ok("LGTM!!".to_string()),
		_ => Err("Boooo..".to_string()),
	}
}

fn main() {

	let args: Vec<String> = std::env::args().collect();

	// 1st argument
	let s = match 2 <= args.len() {
		true => &args[1],
		_ => "",
	};

	// test
	let result = run(s);
	if result.is_err() {
		let answer = result.err().unwrap();
		println!("[TRACE] NG! {}", answer);
		return;
	}

	let response = result.unwrap();
	println!("[TRACE] OK. {}", response);
}
