// use std::fs;
use std::path::Path;
use std::ffi::OsStr;
// use std::env;

extern crate uuid;
use uuid::Uuid;

fn _generate_new_name() -> String {

	let uuid = Uuid::new_v4();
	return uuid.hyphenated().to_string();
}

fn on_file_found(e: &Path) -> std::io::Result<()> {

	let parent = match e.parent() {
		Some(result) => result,
		None => Path::new("")
	};

	let name = _generate_new_name();

	let ext = match e.extension() {
		Some(result1) => result1,
		None => OsStr::new("")
	};

	let new_path = parent.join(&name).with_extension(ext);
	println!("{}", new_path.as_os_str().to_str().unwrap());

	std::fs::rename(e, new_path)?;

	Ok(())
}

#[allow(unused)]
fn enumerate(e: &Path, handler: &Fn(&Path) -> std::io::Result<()>) -> std::io::Result<()> {

	if !e.exists() {
		println!("[TRACE] invalid path {}", e.to_str().unwrap());
		return Ok(());
	}

	if e.is_dir() {
		let it = std::fs::read_dir(e)?;
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			enumerate(&path, handler);
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

	for e in &args[1..] {
		let path = Path::new(e);
		let _ = enumerate(path, &on_file_found);
	}
}
