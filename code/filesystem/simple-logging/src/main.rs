use std::io::Write;

fn trunc_and_write() {

	// always create
	let mut f = std::fs::File::create("main.log").unwrap();
	f.write_all(b"trunc.\n").unwrap();
}

fn append() {

	// create or append
	let mut f = std::fs::OpenOptions::new().create(true).append(true).open("main.log").unwrap();
	f.write_all(b"test.\n").unwrap();
}

fn main() {

	trunc_and_write();
	append();
	append();
	append();
	append();
	append();
}
