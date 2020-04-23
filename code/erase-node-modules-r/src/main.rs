fn remove_dir_all(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	if !std::path::Path::new(path).exists() {
		return Ok(());
	}
	std::fs::remove_dir_all(path)?;
	return Ok(());
}

fn find_file(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[ERROR] invalid path {}", source_path.to_str().unwrap());
		return Ok(());
	}
	if source_path.is_dir() {
		let name = source_path.file_name();
		if name.is_some() && name.unwrap() == "node_modules" {
			println!("delete ... {}", source_path.to_str().unwrap());
			remove_dir_all(source_path.as_os_str().to_str().unwrap())?;
			return Ok(());
		}
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let path = entry.path();
			find_file(&path.as_os_str().to_str().unwrap())?;
		}
		return Ok(());
	}
	return Ok(());
}

fn erase_node_modules_r(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	find_file(path)?;
	Ok(())
}

fn usage() {
	println!("パスを指定します");
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut count = 0;
	for arg in &args[1..] {
		let result = erase_node_modules_r(arg.as_str());
		if result.is_err() {
			println!("[ERROR] {}", result.err().unwrap());
			return;
		}
		count = count + 1;
	}
	if count == 0 {
		usage();
		return;
	}
}
