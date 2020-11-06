extern crate base64;
extern crate chrono;
extern crate ctrlc;
extern crate reqwest;

fn read_text_file(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

fn get_current_timestamp() -> String {
	let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
	return format!("{}", timestamp);
}

fn call_get_content(url: &str, access_token: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// 大きなファイルのコンテンツは直接取得できません。初めにディレクトリ "/" を参照してファイルを列挙してから、SHA を指定してファイルのコンテンツをダウンロードする必要があります。
	println!("{} [TRACE] attack.", get_current_timestamp());
	let client = reqwest::Client::new();
	let access_token_header = format!("token {}", access_token);
	let mut response = client.get(url).header("Accept", "application/vnd.github.v3+json").header("Authorization", access_token_header).send()?;
	let content = response.text()?;
	let body = content;
	// println!("{}", body);

	let value = serde_json::from_str::<serde_json::Value>(body.as_str())?;
	println!("[TRACE] {}", value);
	println!("[TRACE] NAME: {}", value["name"]);
	println!("[TRACE] SHA: {}", value["sha"]);

	let base64_content = value["content"].as_str().unwrap();
	// 改行が入っていると InvalidLength になる
	let base64_content = base64_content.trim();
	let base64 = base64::decode(base64_content)?;
	println!("{:?}", base64);
	let original_content = String::from_utf8(base64)?;
	println!("{}", original_content);

	return Ok(());
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let access_token = read_text_file(".access_token")?;

	let url = "https://api.github.com/repos/mass10/DEPLOY-REPOSITORY/contents/example-01";

	call_get_content(url, &access_token)?;

	return Ok(());
}
