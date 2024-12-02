mod services;

///
/// エントリーポイント
///
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// アプリケーションオブジェクトを初期化します。
	let mut slack = services::SlackClient::new()?;

	// テキストメッセージを投稿します。
	slack.post_text(
		// channel name or channel ID
		"notifications",
		// 書式付きテキストメッセージ
		"テキストメッセージ\r\n`warning`\r\n:four_leaf_clover::four_leaf_clover::four_leaf_clover::four_leaf_clover::four_leaf_clover:",
	)?;

	// DM を送信します。
	slack.post_text(
		// member ID or e-mail
		"member ID or e-mail",
		// テキストメッセージ
		"ダイレクトメッセージです。",
	)?;

	// コメントを付けてファイルを投稿します。
	slack.post_file(
		// channel name or channel ID
		"notifications",
		// テキストメッセージ
		"さあうけとるがよい",
		// ファイルパス
		"0.jpg",
		// ファイル名
		"",
	)?;

	return Ok(());
}
