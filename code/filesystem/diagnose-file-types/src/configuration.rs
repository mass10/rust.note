use toml;

///
pub fn read_text_file_all(path: &str) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

#[allow(unused)]
fn join_as_string(path: &std::path::Path, child: &str) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
	let result = path.join(child);
	let s = result.to_str().unwrap().to_string();
	return Ok(s);
}

fn find_settings_toml() -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
	const NAME: &str = "settings.toml";
	// カレントディレクトリを調べます。
	if std::path::Path::new(NAME).is_file() {
		return Ok(NAME.to_string());
	}
	// みつからない
	return Ok("".to_string());
}

#[derive(serde_derive::Deserialize, std::fmt::Debug, std::clone::Clone)]
pub struct Configuration {
	/// 除外するディレクトリ名
	pub exclude_dirs: Option<std::collections::HashSet<String>>,

	/// 除外するファイル名
	pub exclude_files: Option<std::collections::HashSet<String>>,
}

impl Configuration {
	pub fn new() -> std::result::Result<Configuration, std::boxed::Box<dyn std::error::Error>> {
		// 新しいインスタンス
		let mut conf = Configuration {
			exclude_dirs: Some(std::collections::HashSet::new()),
			exclude_files: Some(std::collections::HashSet::new()),
		};

		let path = find_settings_toml()?;

		// コンフィギュレーション
		conf.configure(&path)?;

		return Ok(conf);
	}

	pub fn get_exclude_dirs(&self) -> &std::collections::HashSet<String> {
		let dirs = self.exclude_dirs.as_ref();
		let dirs = dirs.unwrap();
		return dirs;
	}

	#[allow(unused)]
	pub fn get_exclude_files(&self) -> &std::collections::HashSet<String> {
		let files = self.exclude_files.as_ref();
		let files = files.unwrap();
		return files;
	}

	/// コンフィギュレーション
	fn configure(&mut self, path: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// パスが指定されていなければスキップします。
		if path == "" {
			return Ok(());
		}

		// ファイルが無ければスキップします。
		if !std::path::Path::new(path).is_file() {
			println!("[INFO] Configuration file not found. [{}]", path);
			return Ok(());
		}

		// テキストファイル全体を読み込み
		let content = read_text_file_all(&path)?;

		*self = toml::from_str(&content)?;
		if self.exclude_dirs.is_none() {
			self.exclude_dirs = Some(std::collections::HashSet::new());
		}
		if self.exclude_files.is_none() {
			self.exclude_files = Some(std::collections::HashSet::new());
		}

		return Ok(());
	}
}
