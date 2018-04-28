
fn main() {

	let left = std::path::Path::new("left");
	let right = std::path::Path::new("right");
	let result = std::fs::rename(left, right);
	match result {
		Ok(n) => println!("success: {:?}", n),
		Err(err) => println!("error: {:?}", err)
	}
}
