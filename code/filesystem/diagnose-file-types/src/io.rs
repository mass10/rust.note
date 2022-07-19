use crate::configuration::Configuration;

/// ファイルハンドラー
///
/// ※型に置き換えるとコンパイルエラーになる🔥
#[allow(unused)]
type FileHandler = dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;

fn retrieve_name(path: &std::path::Path) -> Option<String> {
	let pathname = path.canonicalize().unwrap();
	let pathname = pathname.as_path();

	let name = pathname.file_name()?;
	let name = name.to_str()?;
	return Some(name.to_string());
}

/// ファイル走査
///
/// # Arguments
/// * `e` パス
/// * `handler` ファイルハンドラー
pub fn search(conf: &Configuration, path: &std::path::Path, handler: &mut dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	if !path.exists() {
		println!("[TRACE] invalid path {}", path.to_str().unwrap());
		return Ok(());
	}

	if path.is_dir() {
		let name = retrieve_name(path).unwrap();

		// 名前のフィルタリング
		for pat in &conf.exclude_dirs {
			// TODO: ちゃんとする
			if &name == pat {
				return Ok(());
			}
		}

		let it = std::fs::read_dir(path)?;
		for e in it {
			let entry = e.unwrap();
			let entry_path = entry.path();
			search(conf, &entry_path, handler)?;
		}
		return Ok(());
	} else if path.is_file() {
		let name = retrieve_name(path).unwrap();

		// 名前のフィルタリング
		for pat in &conf.exclude_files {
			// TODO: ちゃんとする
			if name.contains(pat) {
				return Ok(());
			}
		}

		return handler(path);
	} else {
		println!("[WARN] 不明なファイルシステム {:?}", path);
	}
	return Ok(());
}
