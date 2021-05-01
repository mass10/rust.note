use crypto::mac::Mac;

extern crate base64;
extern crate crypto;
extern crate hmac;
extern crate reqwest;

pub fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

pub fn parse_json_to_value(json_text: &str) -> std::option::Option<serde_json::Value> {
	let result = serde_json::from_str::<serde_json::Value>(json_text);
	if result.is_err() {
		return None;
	}
	return Some(result.ok().unwrap());
}

fn select(left: &str, right: &str) -> String {
	return if left != "" { left.to_string() } else { right.to_string() };
}

fn digest_sha1(s: &str) -> String {
	use crypto::digest::Digest;

	let mut digest = crypto::sha1::Sha1::new();
	digest.input_str(s);
	return digest.result_str();
}

fn hmac_sha1(data: &str, key: &str) -> String {
	let mut mac = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), key.as_bytes());
	mac.input(data.as_bytes());
	let result = mac.result();
	let code = result.code();
	let code = std::str::from_utf8(&code).unwrap();
	return code.to_string();
}

///
/// コンフィギュレーション
///
pub struct Configuration {
	pub access_key_id: String,
	pub secret_access_key: String,
}

impl Configuration {
	pub fn new() -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
		let conf = Configuration {
			access_key_id: String::new(),
			secret_access_key: String::new(),
		};
		return Ok(conf);
	}

	pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let content = read_text_file(".settings.json")?;

		let settings = parse_json_to_value(&content);
		if settings.is_none() {
			return Ok(());
		}
		let settings = settings.unwrap();

		let access_token = settings["access_key_id"].as_str().unwrap();
		self.access_key_id = access_token.to_string();

		let access_token = settings["secret_access_key"].as_str().unwrap();
		self.secret_access_key = access_token.to_string();

		return Ok(());
	}
}

///
/// Amazon S3 Client
///
pub struct AmazonS3Client {
	conf: std::option::Option<Configuration>,
}

impl AmazonS3Client {
	pub fn new() -> std::result::Result<AmazonS3Client, Box<dyn std::error::Error>> {
		let client = AmazonS3Client { conf: None };
		return Ok(client);
	}

	pub fn configure(&mut self) -> std::result::Result<Box<&Configuration>, Box<dyn std::error::Error>> {
		if self.conf.is_some() {
			let conf = self.conf.as_ref().unwrap();
			return Ok(Box::new(conf));
		}
		let mut conf = Configuration::new()?;
		conf.configure()?;
		self.conf = Some(conf);
		let conf = self.conf.as_ref().unwrap();
		return Ok(Box::new(conf));
	}

	pub fn ls(&mut self, host: &str, bucket_name: &str, key: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let conf = self.configure()?;

		let date = "Sat, 1 May 2021 23:20:00 +0000";
		let date = "2021-05-01 23:20:00 +0900";
		let str_to_sign = ["GET", "", "", date, key].join("\n");

		let sha1 = digest_sha1("");

		let source = format!("{}{}", conf.secret_access_key, str_to_sign);
		let input = hmac_sha1(&source, &conf.secret_access_key);
		let signature = base64::encode(&input);

		let auth_string = format!("AWS {}:{}", conf.access_key_id, signature);

		let client = reqwest::Client::new();
		// let url = "https://s3-ap-northeast-1.amazonaws.com/my-bucket-name/";
		let builder = client.get(url);
		let builder = builder.header("Host", host);
		let builder = builder.header("Authorization", auth_string);
		let mut response = builder.send()?;

		let content = response.text()?;
		println!("{}", content);

		{
			let value = serde_json::from_str::<serde_json::Value>(content.as_str())?;
			println!("[TRACE] {}", value);
			println!("[TRACE] {}", value["error"].as_str().unwrap_or_default());
			println!("[TRACE] {:?}", value["ok"]);
			println!("[TRACE] {:?}", value["response_metadata"]);
		}

		return Ok(());
	}
}

///
/// エントリーポイント
///
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let mut s3 = AmazonS3Client::new()?;

	s3.ls("s3-ap-northeast-1.amazonaws.com", "my-bucket-name", "/key/to/entry")?;

	return Ok(());
}
