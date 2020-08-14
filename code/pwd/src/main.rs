/// エントリーポイント
fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let current_directory = std::env::current_dir()?;
	print!("{}", current_directory.as_path().to_str().unwrap());
	return Ok(());
}
