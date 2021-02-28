/// 指定されたディレクトリーを削除します。
///
/// ### Arguments
/// `path` パス
fn remove_dir_all(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	if !std::path::Path::new(path).exists() {
		return Ok(());
	}
	std::fs::remove_dir_all(path)?;
	return Ok(());
}

/// ディレクトリー配下を走査します。
fn find_file(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(path);
	if !source_path.exists() {
		println!("[ERROR] invalid path {}", source_path.to_str().unwrap());
		return Ok(());
	}

	if source_path.is_dir() {
		let name = source_path.file_name().unwrap().to_str().unwrap();
		if name == "node_modules" {
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

/// node_modules を探して削除します。
///
/// ### Arguments
/// `path` 起点のパス
fn erase_node_modules_r(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	find_file(path)?;
	return Ok(());
}

/// 使用方法
fn usage() {
	println!("node_modules ディレクトリーを削除します。");
	println!("パスを指定します");
}

/// エントリーポイント
fn main() {
	// コマンドライン引数
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		usage();
		std::thread::sleep(std::time::Duration::from_millis(2500));
		return;
	}

	for arg in &args {
		// node_modules ディレクトリをすべて削除
		let result = erase_node_modules_r(arg.as_str());
		if result.is_err() {
			println!("[ERROR] {}", result.err().unwrap());
			return;
		}
	}

	std::thread::sleep(std::time::Duration::from_millis(2500));
}
