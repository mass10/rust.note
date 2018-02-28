use std::io::Read;

fn read_text_file(path: &str) -> String {

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	return s;
}

fn main() {

	let s = read_text_file("src/main.rs");
	println!("{}", s);
}
