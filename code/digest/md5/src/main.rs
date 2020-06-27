extern crate md5;

fn digest_by_md5(s: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let result = md5::compute(s);
	return Ok(format!("{:x}", result));
}

fn test(s: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let digest = digest_by_md5(s)?;
	println!("[TRACE] [{}] >> [{}] (MD5)", s, digest);
	Ok(())
}

fn main() {
	match test("Hello, world!") {
		Err(reason) => {
			println!("[ERROR] {}", reason);
		},
		Ok(_) => {
			println!("Ok.");
		}
	}
}
