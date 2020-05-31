fn read_text_file(path: &str) -> String {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	return s;
}

fn main() {
	// reading whole text file.
	{
		let s = read_text_file("src/main.rs");
		println!("{}", s);
	}

	// reading a text file line-by-line.
	{
		use std::io::BufRead;
		let path = "src/main.rs";
		let file = std::fs::File::open(path).unwrap();
		let r = std::io::BufReader::new(file);
		for e in r.lines() {
			let line = e.unwrap();
			println!("{}", line);
		}
	}
}
