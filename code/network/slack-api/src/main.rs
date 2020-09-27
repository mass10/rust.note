extern crate base64;
extern crate chrono;
extern crate ctrlc;
extern crate reqwest;

mod util {

	/// テキストファイル全体を読み込んで返します。
	pub fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
		use std::io::Read;

		let mut file = std::fs::File::open(path).unwrap();
		let mut s = String::new();
		file.read_to_string(&mut s)?;
		return Ok(s);
	}

	/// 現在のタイムスタンプを返します。
	#[allow(unused)]
	pub fn get_current_timestamp() -> String {
		let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
		return format!("{}", timestamp);
	}

	/// JSON をパースします。
	pub fn parse_json_to_value(json_text: &str) -> std::option::Option<serde_json::Value> {
		let result = serde_json::from_str::<serde_json::Value>(json_text);
		if result.is_err() {
			return None;
		}
		return Some(result.ok().unwrap());
	}
}

mod configuration {

	use super::util;

	/// コンフィギュレーション
	pub struct Configuration {
		access_token: String,
	}

	impl Configuration {
		/// 単純な初期化
		pub fn new() -> Configuration {
			let conf = Configuration { access_token: String::new() };
			return conf;
		}

		/// コンフィギュレーション
		pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			let content = util::read_text_file(".settings.json")?;
			let settings = util::parse_json_to_value(&content);
			if settings.is_none() {
				return Ok(());
			}
			let settings = settings.unwrap();
			let access_token = settings["access_token"].as_str().unwrap();
			self.access_token = access_token.to_string();
			return Ok(());
		}

		/// アクセストークンを返します。
		pub fn get_access_token(&self) -> String {
			return self.access_token.clone();
		}
	}
}

mod application {

	use super::configuration;

	/// アプリケーション
	pub struct Application {
		conf: std::option::Option<configuration::Configuration>,
	}

	impl Application {
		/// アプリケーションのインスタンスを返します。
		pub fn new() -> std::result::Result<Application, Box<dyn std::error::Error>> {
			let app = Application { conf: None };
			return Ok(app);
		}

		/// コンフィギュレーション
		pub fn configure(&mut self) -> std::result::Result<Box<&configuration::Configuration>, Box<dyn std::error::Error>> {
			if self.conf.is_some() {
				let conf = self.conf.as_ref().unwrap();
				return Ok(Box::new(conf));
			}
			let mut conf = configuration::Configuration::new();
			conf.configure()?;
			self.conf = Some(conf);
			let conf = self.conf.as_ref().unwrap();
			return Ok(Box::new(conf));
		}

		/// コメントを付けてファイルを投稿します。
		pub fn upload_file(&mut self, text: &str, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
			let conf = self.configure()?;
			let access_token = conf.get_access_token();

			let url = "https://slack.com/api/files.upload";
			let client = reqwest::Client::new();
			let access_token_header = format!("Bearer {}", access_token);

			let form = reqwest::multipart::Form::new()
				.text("initial_comment", text.to_string())
				.text("channels", "notifications")
				.file("file", path)?;

			let mut response = client
				.post(url)
				.header("Content-Type", "multipart/form-data")
				.header("Authorization", access_token_header)
				.multipart(form)
				.send()?;
			let content = response.text()?;
			let body = content;
			println!("{}", body);

			if false {
				let value = serde_json::from_str::<serde_json::Value>(body.as_str())?;
				println!("[TRACE] {}", value);
				println!("[TRACE] NAME: {}", value["name"]);
				println!("[TRACE] SHA: {}", value["sha"]);
			}

			return Ok(());
		}
	}
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// アプリケーションオブジェクトを初期化します。
	let mut app = application::Application::new()?;

	// コメントを付けてファイルを投稿します。
	app.upload_file("さあうけとるがよい", "0.jpg")?;

	return Ok(());
}
