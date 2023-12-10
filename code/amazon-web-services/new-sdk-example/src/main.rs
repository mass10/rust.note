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
