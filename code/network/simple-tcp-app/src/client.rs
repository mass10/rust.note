/// メッセージ送信側
pub fn try_to_send_message() -> Result<(), Box<dyn std::error::Error>> {
	use std::io::Write;

	// サーバーに接続を試みます。
	let result = std::net::TcpStream::connect("127.0.0.1:8082");
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] サーバーに接続できません。情報: {}", error);
		return Ok(());
	}
	let mut connection = result.unwrap();

	let message = "{\"message_method\": \"HELLO\", \"content\": \"Hello, World!!\"}";
	connection.write_all(message.as_bytes())?;
	// connection.shutdown(std::net::Shutdown::Both).expect("shutdown call failed");
	connection.shutdown(std::net::Shutdown::Both)?;

	return Ok(());
}
