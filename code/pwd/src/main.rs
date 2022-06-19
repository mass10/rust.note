/// エントリーポイント
fn main() {
	let result = std::env::current_dir();
	let current_directory = result.unwrap();
	print!("{}", current_directory.as_path().to_str().unwrap());
}
