struct Configuration {}

impl Configuration {
	fn new() -> Configuration {
		return Configuration {};
	}
}

fn get_file_name(path: &std::path::Path) -> &str {
	let name = path.file_name();
	if name.is_none() {
		return "";
	}
	let name = name.unwrap().to_str();
	if name.is_none() {
		return "";
	}
	return name.unwrap();
}

fn get_parent_file_name(path: &std::path::Path) -> &str {
	let parent = path.parent();
	if parent.is_none() {
		return "";
	}
	return get_file_name(parent.unwrap());
}

/// ファイルハンドラー
///
/// ※型に置き換えるとコンパイルエラーになる🔥
#[allow(unused)]
type FileHandler = dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;

/// ファイル走査
///
/// # Arguments
/// * `e` パス
/// * `handler` ファイルハンドラー
fn search(conf: &Configuration, path: &str, handler: &mut dyn FnMut(&str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let unknown = std::path::Path::new(path);
	if !unknown.exists() {
		// println!("[TRACE] invalid path {}", path.to_str().unwrap());
		return Ok(());
	}

	// println!("[TRACE] FOUND [{}]", path.to_str().unwrap());

	if unknown.is_dir() {
		// 親ディレクトリの名前
		let parent_directory_name = get_parent_file_name(&unknown);
		// このディレクトリの名前
		let directory_name = get_file_name(unknown);

		if parent_directory_name == "" {
			// ドライブ、もしくは第一階層 (親なし)
			if directory_name == "" {
				// ドライブ
				println!("[DEBUG] ドライブ [{}] -> [{}] ({})", parent_directory_name, path, directory_name);
			} else if directory_name.contains("Program Files") {
				println!("[DEBUG] ドライブ [{}] -> [{}] ({})", parent_directory_name, path, directory_name);
			} else {
				// 第一階層
				// ここはもう掘らない
				return Ok(());
			}
		} else if parent_directory_name.contains("Program Files") {
			// Program Files* の直下
			if directory_name.contains("MSBuild") {
				println!("[DEBUG] その他のディレクトリ [{}] -> [{}]", parent_directory_name, path);
			} else if directory_name.contains("Microsoft Visual Studio") {
				println!("[DEBUG] その他のディレクトリ [{}] -> [{}]", parent_directory_name, path);
			} else {
				// ここはもう掘らない
				return Ok(());
			}
		} else {
			// println!("[DEBUG] ？ [{}] -> [{}]", parent_directory_name, path);
		}

		let it = std::fs::read_dir(path)?;
		for e in it {
			let entry = e.unwrap();
			let entry_path = entry.path();
			let entry_path = entry_path.to_str().unwrap();
			let _result = search(conf, &entry_path, handler);
		}
		return Ok(());
	} else if unknown.is_file() {
		// let name = path.file_name().unwrap();
		// let name = retrieve_name(path).unwrap();

		return handler(path);
	} else {
		println!("[WARN] 不明なファイルシステム {:?}", path);
	}
	return Ok(());
}

fn main() {
	let conf = Configuration::new();

	let path = "C:\\";

	// ハンドラー
	let mut handler = |arg: &str| -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		let unknown = std::path::Path::new(arg);
		if !unknown.is_file() {
			return Ok(());
		}
		let file_name = get_file_name(unknown);
		// let path_as_str = arg;
		if !file_name.ends_with(".exe") {
			return Ok(());
		}
		if !file_name.contains("MSBuild") {
			return Ok(());
		}
		println!("[DEBUG] ファイルを検出 [{}]", arg);
		return Ok(());
	};

	let result = search(&conf, path, &mut handler);
	if result.is_err() {
		let err = result.err().unwrap();
		println!("ERROR: {}", err);
		return;
	}
}
