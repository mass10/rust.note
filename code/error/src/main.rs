
fn main() {

	let left = std::path::Path::new("left");
	let right = std::path::Path::new("right");

	// rename() はエラー情報を返すが、拾わなくても panic しない
	let result = std::fs::rename(left, right);
	match result {
		Ok(n) => println!("success: {:?}", n),
		Err(err) => println!("error: {:?}", err),
	}
}
