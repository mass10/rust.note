/// エントリーポイント
fn main() {
	let result = std::env::current_dir();
	if result.is_err() {
		println!("{}", result.unwrap_err());
		return;
	}

	let current_directory = result.unwrap();

	print!("{}", current_directory.as_path().to_str().unwrap());
}
