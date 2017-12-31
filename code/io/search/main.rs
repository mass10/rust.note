// use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;

fn on_entry(e: &DirEntry) {

	let path_str = String::from(e.path().as_os_str().to_str().unwrap().to_string());
	println!("{}", path_str);
}

# [allow(unused)]
fn search(dir: &Path, cb: &Fn(&DirEntry)) {

	if dir.is_dir() {
		let it = fs::read_dir(dir).unwrap();
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			if path.is_dir() {
				search(&path, cb);
			} else {
				cb(&entry);
			}
		}
	}
}

fn main() {

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}
	let path = Path::new(&args[1]);
	search(path, &on_entry);
}
