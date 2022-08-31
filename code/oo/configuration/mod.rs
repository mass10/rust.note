//!
//! コンフィギュレーション
//!

/// コンフィギュレーションインターフェイス
pub trait ConfigurationSettings {
	fn get_field1(&self) -> &String;
	fn get_request_string(&self) -> &String;
}

/// コンフィギュレーション
pub fn configure() -> std::result::Result<Box<dyn ConfigurationSettings>, Box<dyn std::error::Error>> {
	let mut instance = ConfigurationSettingsImpl {
		field1: String::new(),
		request_string: String::new(),
	};

	let args: Vec<String> = std::env::args().skip(1).collect();
	if 0 < args.len() {
		instance.request_string = args[0].clone();
	}

	return Ok(Box::new(instance));
}

/// コンフィギュレーション構造体と実装の定義(非公開)
#[derive(Debug)]
struct ConfigurationSettingsImpl {
	/// field1
	field1: String,
	/// リクエスト文字列
	request_string: String,
}

impl ConfigurationSettings for ConfigurationSettingsImpl {
	fn get_field1(&self) -> &String {
		return &self.field1;
	}

	fn get_request_string(&self) -> &String {
		return &self.request_string;
	}
}
