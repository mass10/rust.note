mod application;

/// エントリーポイントです。
fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		let result = application::stat(e.as_str());
		if result.is_err() {
			println!("[ERROR] <main()> {}", result.err().unwrap());
		}
	}
}
