extern crate serde;
extern crate serde_derive;
extern crate toml;

#[derive(serde_derive::Deserialize)]
struct User {
	#[allow(unused)]
	email: String,
	#[allow(unused)]
	name: Option<String>,
	#[allow(unused)]
	age: Option<u16>,
}

fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

fn read_toml_file(path: &str) -> std::result::Result<User, Box<dyn std::error::Error>> {
	let content = read_text_file_all(path)?;
	let user: User = toml::from_str(&content)?;
	return Ok(user);
}

fn test01() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// TOML ファイル読み込み
	let user = read_toml_file("settings.toml")?;

	// ダンプ
	println!("[TRACE] DUMP");
	println!("[TRACE] {:?}", user.email);
	println!("[TRACE] {:?}", user.name);
	println!("[TRACE] {:?}", user.age);
	println!("[TRACE] Ok.");

	return Ok(());
}

fn main() {
	let result = test01();
	if result.is_err() {
		println!("[ERROR] reason: {:?}", result.unwrap());
	}
}
