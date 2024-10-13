extern crate reqwest;

mod services;

///
/// エントリーポイント
///
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// アプリケーションオブジェクトを初期化します。
	let mut slack = services::SlackClient::new()?;

	// テキストメッセージを投稿します。
	slack.post_text(
		"notifications",
		"テキストメッセージ\r\n`warning`\r\n:four_leaf_clover::four_leaf_clover::four_leaf_clover::four_leaf_clover::four_leaf_clover:",
	)?;

	// コメントを付けてファイルを投稿します。
	slack.upload_file("notifications", "さあうけとるがよい", "0.jpg", "")?;

	return Ok(());
}
