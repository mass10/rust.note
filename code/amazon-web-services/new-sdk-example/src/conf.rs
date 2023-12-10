//!
//! 設定ファイル関連の操作を提供します。
//!

/// 設定ファイルの内容
#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, std::clone::Clone)]
pub struct ConfigurationSettings {
	pub aws_access_key_id: String,
	pub aws_secret_access_key: String,
	pub aws_session_token: String,
}

impl ConfigurationSettings {
	/// 新しいインスタンスを返します。
	pub fn new() -> Result<ConfigurationSettings, Box<dyn std::error::Error>> {
		const CONF_NAME: &str = ".conf.toml";
		let content = std::fs::read_to_string(CONF_NAME)?;
		let conf: ConfigurationSettings = toml::from_str(&content)?;
		return Ok(conf);
	}
}
