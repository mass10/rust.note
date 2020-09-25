extern crate base64;
extern crate chrono;
extern crate ctrlc;
extern crate reqwest;

struct Util {}

impl Util {
	pub fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
		use std::io::Read;

		let mut file = std::fs::File::open(path).unwrap();
		let mut s = String::new();
		file.read_to_string(&mut s)?;
		return Ok(s);
	}

	pub fn get_current_timestamp() -> String {
		let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
		return format!("{}", timestamp);
	}

	pub fn parse_json_to_value(json_text: &str) -> std::option::Option<serde_json::Value> {
		let result = serde_json::from_str::<serde_json::Value>(json_text);
		if result.is_err() {
			return None;
		}
		return Some(result.ok().unwrap());
	}
}

struct Configuration {
	access_token: String,
}

impl Configuration {
	// 単純な初期化
	pub fn new() -> Configuration {
		let conf = Configuration { access_token: String::new() };
		return conf;
	}
	/// コンフィギュレーション
	pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let content = Util::read_text_file(".settings.json")?;
		let settings = Util::parse_json_to_value(&content);
		if settings.is_none() {
			return Ok(());
		}
		let settings = settings.unwrap();
		self.access_token = settings["access_token"].to_string();
		return Ok(());
	}
}

/// アプリケーション
struct Application {
	conf: std::option::Option<Configuration>,
}

/// アプリケーション
impl Application {
	pub fn new() -> std::result::Result<Application, Box<dyn std::error::Error>> {
		let app = Application { conf: None };
		return Ok(app);
	}

	/// コンフィギュレーション
	pub fn configure(&mut self) -> std::result::Result<Box<&Configuration>, Box<dyn std::error::Error>> {
		if self.conf.is_some() {
			let conf = self.conf.as_ref().unwrap();
			return Ok(Box::new(conf));
		}
		let mut conf = Configuration::new();
		conf.configure()?;
		self.conf = Some(conf);
		let conf = self.conf.as_ref().unwrap();
		return Ok(Box::new(conf));
	}

	pub fn upload_file(&mut self, text: &str, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		println!("{} [TRACE] attack.", Util::get_current_timestamp());
		println!("{} [TRACE] text: [{}]", Util::get_current_timestamp(), text);
		println!("{} [TRACE] file: [{}]", Util::get_current_timestamp(), path);

		// コンフィギュレーション
		let conf = self.configure()?;
		// let conf = self.configure()?;
		// ファイルアップロード API の URL
		let url = "https://slack.com/api/files.upload";
		// HTTP クライアント
		let client = reqwest::Client::new();
		// アクセストークン
		let access_token_header = format!("Bearer {}", conf.access_token);
		// POST
		let mut response = client
			// HTTP Client
			.post(url)
			// コンテントタイプ
			.header("Content-Type", "multipart/form-data")
			// 認可ヘッダー
			.header("Authorization", access_token_header)
			.send()?;
		let content = response.text()?;
		let body = content;
		// println!("{}", body);

		if false {
			let value = serde_json::from_str::<serde_json::Value>(body.as_str())?;
			println!("[TRACE] {}", value);
			println!("[TRACE] NAME: {}", value["name"]);
			println!("[TRACE] SHA: {}", value["sha"]);
		}

		return Ok(());
	}
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let mut app = Application::new()?;
	app.conf = None;
	app.upload_file("さあうけとるがよい", "run.bat")?;
	return Ok(());
}
