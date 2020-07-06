use std::io::Write;

fn trunc_and_write() {
	// always create
	let mut f = std::fs::File::create("main.log").unwrap();
	f.write_all(b"trunc.\n").unwrap();
}

fn append(text: &str) {
	// create or append
	let mut f = std::fs::OpenOptions::new().create(true).append(true).open("main.log").unwrap();
	f.write_all(text.as_bytes()).unwrap();
	f.write_all(b"\n").unwrap();
}

fn main() {
	append("000");
	trunc_and_write();
	append("aaa");
	append("bbb");
	append("ccc");
}
