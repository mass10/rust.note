use crate::configuration::Configuration;

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
pub fn search(conf: &Configuration, path: &std::path::Path, handler: &mut dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	if !path.exists() {
		println!("[TRACE] invalid path {}", path.to_str().unwrap());
		return Ok(());
	}

	if path.is_dir() {
		let pathname = path.canonicalize().unwrap();
		let _pathname = pathname.as_os_str().to_str().unwrap();

		let name = path.file_name().unwrap_or_default();
		let name = name.to_str().unwrap();

		// 名前のフィルタリング
		for e in conf.get_exclude_dirs() {
			// TODO: ちゃんとする
			if name == e {
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
		let name = path.file_name().unwrap_or_default();
		let name = name.to_str().unwrap();

		// 名前のフィルタリング
		for pat in conf.get_exclude_files() {
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
