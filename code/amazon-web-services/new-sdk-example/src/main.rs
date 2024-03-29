//!
//! アプリケーションのエントリーポイントを定義しています。
//!

mod conf;
mod util;

/// クライアントオブジェクトを生成します。
async fn create_s3_client(_: &conf::ConfigurationSettings) -> Result<aws_sdk_s3::Client, Box<dyn std::error::Error>> {
	let behavior = aws_config::BehaviorVersion::v2023_11_09();
	let region = aws_sdk_s3::config::Region::new("ap-northeast-1");
	let config = aws_config::defaults(behavior).region(region).load().await;
	let client = aws_sdk_s3::Client::new(&config);
	return Ok(client);
}

/// ミリ秒を [std::time::SystemTime] に変換します。
fn millis_to_systemtime(millisec: i64) -> std::io::Result<std::time::SystemTime> {
	let duration = std::time::Duration::from_millis(millisec as u64);
	let systemtime = std::time::UNIX_EPOCH + duration;
	return Ok(systemtime);
}

/// ファイルの更新日時を変更します。
fn set_filetime(path: &str, time: &std::time::SystemTime) -> Result<(), Box<dyn std::error::Error>> {
	let mtime: filetime::FileTime = filetime::FileTime::from_system_time(*time);
	filetime::set_file_mtime(path, mtime)?;
	return Ok(());
}

/// オブジェクトをダウンロードします。
async fn download_object(mut object: aws_sdk_s3::operation::get_object::GetObjectOutput, path: &str) -> Result<(), Box<dyn std::error::Error>> {
	// ファイルサイズを取得
	let object_length = object.content_length.unwrap_or(0);

	// ファイルに保存
	{
		let mut file = std::fs::File::create(path)?;
		let mut byte_count = 0_usize;
		while let Some(bytes) = object.body.try_next().await? {
			use std::io::Write;

			let bytes_len = bytes.len();
			file.write_all(&bytes)?;
			byte_count += bytes_len;
		}
		// ファイルサイズを確認
		if byte_count != object_length as usize {
			return Err("サイズ不一致".into());
		}
	}

	// ファイルタイムをコピー
	{
		let filetime = object.last_modified.unwrap();
		let millisec = filetime.to_millis().unwrap();
		let systemtime = millis_to_systemtime(millisec)?;
		set_filetime(path, &systemtime)?;
	}

	return Ok(());
}

/// アプリケーションを実行します。
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
	// コンフィギュレーション
	let conf = conf::ConfigurationSettings::new()?;

	// AWS クレデンシャル情報を環境変数に露出させます。
	util::setenv("AWS_ACCESS_KEY_ID", &conf.aws_access_key_id);
	util::setenv("AWS_SECRET_ACCESS_KEY", &conf.aws_secret_access_key);
	util::setenv("AWS_SESSION_TOKEN", &conf.aws_session_token);

	// AWS クライアントを初期化します。
	let client = create_s3_client(&conf).await?;
	let name = "my-bucket-031a7053-4f7d-4970-b07f-2f7f1089693f.s3.amazonaws.com";
	let result = client.list_objects_v2().bucket(name).prefix("a/b/c/").send().await?;

	// エントリーを列挙します。
	let contents = result.contents;
	let objects = contents.unwrap();
	for content in &objects {
		println!("{:?}", content);
	}

	// オブジェクトをダウンロードします。
	let object = client
		.get_object()
		.bucket(name)
		.key("a/b/c.unknown")
		.response_content_type("application/zip")
		.send()
		.await?;
	download_object(object, ".tmp\\download.tmp").await?;

	return Ok(());
}

/// Rust アプリケーションのエントリーポイントです。
#[tokio::main]
async fn main() {
	info!("##### START #####");

	let result = run().await;
	if result.is_err() {
		let err = result.err().unwrap();
		let message = format!("{}", err);
		if message != "" {
			error!("{}", err);
		}
		return;
	}

	info!("----- End -----");
}
