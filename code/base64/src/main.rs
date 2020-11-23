extern crate base64;

#[allow(unused)]
fn encode_base64(s: &str) -> String {
	return base64::encode(s);
}

fn main() {
	println!("[{}]", base64::encode("{\"alg\":\"RS256\",\"typ\":\"JWT\"}"));
	println!("[{}]", base64::encode("Hello, world!"));
}
