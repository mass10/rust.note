use std::fs::File;
use std::io::Write;

fn test1() {

	// always create
	let mut f = File::create("main.log").unwrap();
	f.write_all(b"test.\n").unwrap();
}

fn test2() {

	// create or append
	let mut f = std::fs::OpenOptions::new().create(true).append(true).open("main.log").unwrap();
	f.write_all(b"test.\n").unwrap();
}

fn main() {

	// test1();
	test2();
}
