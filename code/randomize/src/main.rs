// use std::fs;
use std::path::Path;
// use std::env;

extern crate uuid;
use uuid::Uuid;

fn _gen() -> String {

	let uuid = Uuid::new_v4();
	return uuid.hyphenated().to_string();
}

fn on_file_found(e: &Path) -> std::io::Result<()> {

	let path_str = e.as_os_str().to_str().unwrap();
	let parent = match e.parent() {
		Some(result) => result,
		None => Path::new("")
	};
	let name = ""; //e.file_stem().unwrap().to_str().unwrap();
	let extension = ""; //e.extension().unwrap().to_str().unwrap();
	println!("{}, {}, {}", parent.as_os_str().to_str().unwrap(), name, extension);
	Ok(())
}

# [allow(unused)]
fn search(e: &Path, handler: &Fn(&Path) -> std::io::Result<()>) -> std::io::Result<()> {

	if !e.exists() {
		println!("[TRACE] invalid path {}", e.to_str().unwrap());
		return Ok(());
	}
	if e.is_dir() {
		let it = std::fs::read_dir(e)?;
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			search(&path, handler);
		}
	}
	else {
		return handler(e);
	}
	Ok(())
}

fn main() {

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}
	let path = Path::new(&args[1]);
	let _result = search(path, &on_file_found);
}
