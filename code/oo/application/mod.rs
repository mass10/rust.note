//!
//! アプリケーション
//!

use super::configuration;

/// アプリケーションのインターフェイスを定義しています。
pub trait Application {
	/// アプリケーションを起動します。
	fn run(&self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;

	/// field1 の値を変更します。
	fn set_field1(&mut self, value: String);

	/// field1 の値を返します。
	fn get_field1(&self) -> &String;
}

/// アプリケーションを初期化して返します。
pub fn create_new(conf: Box<dyn configuration::ConfigurationSettings>) -> Box<dyn Application> {
	// アプリケーション構造体のインスタンスを生成
	let instance = ApplicationImpl {
		field1: conf.get_field1().clone(),
		conf: &conf,
	};
	return Box::new(instance);
}

/// アプリケーション構造体と実装の定義(非公開)
#[derive(Debug)]
struct ApplicationImpl<'a> {
	conf: &'a Box<dyn configuration::ConfigurationSettings>,
	field1: String,
}

impl<'a> Application for ApplicationImpl<'a> {
	/// アプリケーションを起動します。
	fn run(&self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		println!("### START ###");
		// println!("{}", self.get_id());
		println!("--- END ---");

		return Ok(());
	}

	/// field1 の値を変更します。
	fn set_field1(&mut self, value: String) {
		self.field1 = value;
	}

	/// field1 の値を返します。
	fn get_field1(&self) -> &String {
		return &self.field1;
	}
}

impl<'a> Drop for ApplicationImpl<'a> {
	/// 解放
	fn drop(&mut self) {
		println!("(アプリケーションの解放)");
	}
}
