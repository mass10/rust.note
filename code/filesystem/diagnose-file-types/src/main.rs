//!
//! ファイルの拡張子を分析する
//!

extern crate serde_derive;
use toml;

pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

#[allow(unused)]
fn join_as_string(path: &std::path::Path, child: &str) -> Result<String, Box<dyn std::error::Error>> {
	let result = path.join(child);
	let s = result.to_str().unwrap().to_string();
	return Ok(s);
}

fn find_settings_toml() -> Result<String, Box<dyn std::error::Error>> {
	const NAME: &str = "settings.toml";
	// カレントディレクトリを調べます。
	if std::path::Path::new(NAME).is_file() {
		return Ok(NAME.to_string());
	}
	// みつからない
	return Ok("".to_string());
}

#[derive(serde_derive::Deserialize, std::fmt::Debug, std::clone::Clone)]
struct Configuration {
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
	fn configure(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
fn search(conf: &Configuration, path: &std::path::Path, handler: &mut dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
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

/// 拡張子診断クラス
struct Calculator<'a> {
	/// コンフィギュレーションへの参照
	conf: &'a Configuration,
	/// 拡張子と件数を管理します。
	map: std::collections::HashMap<String, u32>,
}

impl<'a> Calculator<'a> {
	/// `Calculator` の新しいインスタンスを返します。
	///
	/// # Returns
	/// `Calculator` の新しいインスタンス
	pub fn new(conf: &'a Configuration) -> Self {
		return Self { conf: conf, map: std::collections::HashMap::new() };
	}

	/// 診断結果を出力します。
	pub fn summary(&mut self) {
		let mut total: u32 = 0;
		for e in &self.map {
			println!("{}\t{}", e.0, e.1);
			total += e.1;
		}
		println!("TOTAL: {:?}", total);
	}

	/// ファイルパスを診断します。
	///
	/// # Arguments
	/// `path` ファイルのパス
	pub fn diagnose(&mut self, path: &std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		let _conf = self.conf;

		// ファイル名
		let name = path.file_name().unwrap_or_default();
		let name = name.to_str();
		if name.is_none() {
			println!("[WARN] Cannot retrieve file name of [{:?}].", path);
			return Ok(());
		}
		let name = name.unwrap();

		// 拡張子
		let result = path.extension();
		if result.is_none() {
			// println!("[WARN] Cannot retrieve ext of [{:?}].", path);
			return Ok(());
		}
		let extension = result.unwrap();
		let result = extension.to_str();
		if result.is_none() {
			println!("[WARN] Cannot retrieve ext of [{:?}].", extension);
			return Ok(());
		}
		let extension = result.unwrap();

		println!("> [{}] >> [{}] + [{}]", path.as_os_str().to_str().unwrap(), name, extension);

		let value = self.map.get_key_value(extension);
		if value.is_none() {
			// 初めての拡張子
			self.map.insert(extension.to_string(), 1);
		} else {
			// 既知の拡張子
			let current = value.unwrap().1;
			let new_value = *current;
			self.map.insert(extension.to_string(), new_value + 1);
		}
		return Ok(());
	}
}

/// エントリーポイント
fn main() {
	// コマンドライン引数
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_millis(1000));
		return;
	}
	let path_to_target = &args[0];
	let path = std::path::Path::new(path_to_target);

	// コンフィギュレーション
	let result = Configuration::new();
	if result.is_err() {
		println!("[ERROR] {:?}", result.unwrap_err());
		return;
	}
	let conf = result.unwrap();

	// 計算機
	let mut calculator = Calculator::new(&conf);

	// ハンドラー
	let mut handler = |arg: &std::path::Path| -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		calculator.diagnose(arg)?;
		return Ok(());
	};

	// ファイル走査
	let result = search(&conf, &path, &mut handler);
	if result.is_err() {
		println!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(3));
		return;
	}

	// サマリーを表示
	calculator.summary();
}
