extern crate reqwest;

///
/// 各種ユーティリティを提供します。
///
mod util {

	/// テキストファイル全体を読み込んで返します。
	///
	/// ### Arguments
	/// * `path` ファイルへのパス
	///
	/// ### Returns
	/// ファイル全体の内容を返します。
	pub fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
		use std::io::Read;

		let mut file = std::fs::File::open(path).unwrap();
		let mut s = String::new();
		file.read_to_string(&mut s)?;
		return Ok(s);
	}

	/// JSON をパースします。
	///
	/// ### Arguments
	/// * `json_text` JSON 文字列
	/// ### Returns
	/// `serde_json::Value`
	pub fn parse_json_to_value(json_text: &str) -> std::option::Option<serde_json::Value> {
		let result = serde_json::from_str::<serde_json::Value>(json_text);
		if result.is_err() {
			return None;
		}
		return Some(result.ok().unwrap());
	}

	/// ファイル、あるいはディレクトリーの名前部分を返します。
	///
	/// ### Arguments
	/// * `path` ファイルのパス
	/// ### Returns
	/// ファイル名
	pub fn get_file_name(path: &str) -> String {
		let file = std::path::Path::new(path);
		return file.file_name().unwrap().to_str().unwrap().to_string();
	}
}

///
/// コンフィギュレーション関連操作を提供します。
///
mod configuration {

	use super::util;

	///
	/// コンフィギュレーション
	///
	pub struct Configuration {
		/// Slack BOT の access_token
		pub access_token: String,
		/// 投稿チャネル
		pub channel: String,
	}

	impl Configuration {
		/// コンフィギュレーションオブジェクトを初期化します。
		///
		/// ### Returns
		/// `Configuration` の新しいインスタンスを返します。
		pub fn new() -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
			let conf = Configuration {
				access_token: String::new(),
				channel: "".to_string(),
			};
			return Ok(conf);
		}

		/// コンフィギュレーションを行います。
		pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			let content = util::read_text_file(".settings.json")?;
			let settings = util::parse_json_to_value(&content);
			if settings.is_none() {
				return Ok(());
			}
			let settings = settings.unwrap();
			let access_token = settings["access_token"].as_str().unwrap();
			let channel = settings["channel"].as_str().unwrap();
			self.access_token = access_token.to_string();
			self.channel = channel.to_string();
			return Ok(());
		}
	}
}

//
// アプリケーション本体に関連した操作を提供します。
//
mod application {

	use super::configuration;
	use super::util;

	///
	/// アプリケーション
	///
	pub struct Application {
		conf: std::option::Option<configuration::Configuration>,
	}

	impl Application {
		/// アプリケーションのインスタンスを返します。
		///
		/// ### Returns
		/// `Application` の新しいインスタンス
		pub fn new() -> std::result::Result<Application, Box<dyn std::error::Error>> {
			let app = Application { conf: None };
			return Ok(app);
		}

		/// コンフィギュレーション
		///
		/// ### Returns
		/// 内部で保持している `configuration::Configuration` への参照
		pub fn configure(&mut self) -> std::result::Result<Box<&configuration::Configuration>, Box<dyn std::error::Error>> {
			if self.conf.is_some() {
				let conf = self.conf.as_ref().unwrap();
				return Ok(Box::new(conf));
			}
			let mut conf = configuration::Configuration::new()?;
			conf.configure()?;
			self.conf = Some(conf);
			let conf = self.conf.as_ref().unwrap();
			return Ok(Box::new(conf));
		}

		/// コメントを付けてファイルを投稿します。
		///
		/// ### Arguments
		/// * `text` コメント
		/// * `path` ファイルへのパス
		pub fn upload_file(&mut self, text: &str, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
			let conf = self.configure()?;

			let url = "https://slack.com/api/files.upload";
			let client = reqwest::Client::new();
			let access_token_header = format!("Bearer {}", conf.access_token);

			let form = reqwest::multipart::Form::new()
				.text("initial_comment", text.to_string())
				.text("channels", conf.channel.to_string())
				.text("title", util::get_file_name(path))
				.file("file", path)?;

			let mut response = client
				.post(url)
				.header("Content-Type", "multipart/form-data")
				.header("Authorization", access_token_header)
				.multipart(form)
				.send()?;

			let content = response.text()?;
			println!("{}", content);

			// JSON を分解してフィールドを読み取る場合
			if true {
				let value = serde_json::from_str::<serde_json::Value>(content.as_str())?;
				println!("[TRACE] {}", value);
				println!("[TRACE] {}", value["error"].as_str().unwrap_or_default());
				println!("[TRACE] {:?}", value["ok"]);
				println!("[TRACE] {:?}", value["response_metadata"]);
			}

			return Ok(());
		}
	}
}

///
/// エントリーポイント
///
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// アプリケーションオブジェクトを初期化します。
	let mut app = application::Application::new()?;

	// コメントを付けてファイルを投稿します。
	app.upload_file("さあうけとるがよい", "0.jpg")?;

	return Ok(());
}
