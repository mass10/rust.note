fn on_entry(e: &std::path::Path) -> std::io::Result<()> {
	let path_str = e.as_os_str().to_str().unwrap();
	println!("{}", path_str);
	return Ok(());
}

#[allow(unused)]
fn search(e: &std::path::Path, handler: &dyn Fn(&std::path::Path) -> std::io::Result<()>) -> std::io::Result<()> {
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
		return Ok(());
	} else if e.is_file() {
		return handler(e);
	} else {
		println!("[WARN] 不明なファイルシステム {:?}", e);
	}
	return Ok(());
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}
	let path = std::path::Path::new(&args[1]);
	let result = search(path, &on_entry);
	if result.is_err() {
		println!("[ERROR] {:?}", result.err());
		return;
	}
}
