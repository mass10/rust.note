use toml;

/// テキストファイル全体を読み込みます。
///
/// # Arguments
/// * `path` ファイルのパス
///
/// # Returns
/// ファイルの内容
pub fn read_text_file_all(path: &str) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

/// パスの連結
#[allow(unused)]
fn join_as_string(path: &std::path::Path, child: &str) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
	let result = path.join(child);
	let s = result.to_str().unwrap().to_string();
	return Ok(s);
}

/// 設定ファイルのパスを検出します。
///
/// # Returns
/// 設定ファイルのパス、存在しない場合は ""
fn find_settings_toml() -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
	const NAME: &str = "settings.toml";

	// カレントディレクトリを調べます。
	if std::path::Path::new(NAME).is_file() {
		return Ok(NAME.to_string());
	}

	// なし
	return Ok("".to_string());
}

/// settings.toml をパースします。
///
/// # Arguments
/// * `path` ファイルのパス
///
/// # Returns
/// `SettingsToml` 構造体
fn parse_settings_toml(path: &str) -> std::result::Result<SettingsToml, std::boxed::Box<dyn std::error::Error>> {
	// テキストファイル全体を読み込み
	let content = read_text_file_all(&path)?;
	// toml をパース
	let settings: SettingsToml = toml::from_str(&content)?;

	return Ok(settings);
}

// TOML の内容
#[derive(serde_derive::Deserialize, std::fmt::Debug, std::clone::Clone)]
struct SettingsToml {
	/// 除外するディレクトリ名
	pub exclude_dirs: std::collections::HashSet<String>,

	/// 除外するファイル名
	pub exclude_files: std::collections::HashSet<String>,
}

/// コンフィギュレーションクラス
#[derive(std::fmt::Debug)]
pub struct Configuration {
	/// 走査するパス
	pub path_to_run: String,

	/// 除外するディレクトリ名
	pub exclude_dirs: std::collections::HashSet<String>,

	/// 除外するファイル名
	pub exclude_files: std::collections::HashSet<String>,
}

impl Configuration {
	/// コンフィギュレーションを行います。
	pub fn new() -> std::result::Result<Configuration, std::boxed::Box<dyn std::error::Error>> {
		// コマンドライン引数
		let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
		let arg = if 0 < args.len() { &args[0] } else { "" };

		// 設定ファイルのパス
		let path = find_settings_toml()?;

		// settings.toml をパースして構造体に変換します。
		let settings = parse_settings_toml(&path)?;

		// 新しいインスタンスを返します。
		let conf = Configuration {
			path_to_run: String::from(arg),
			exclude_dirs: settings.exclude_dirs,
			exclude_files: settings.exclude_files,
		};

		return Ok(conf);
	}
}
