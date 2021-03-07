extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod application_defined_errors {
	#[derive(Debug, Clone)]
	pub struct ApplicationError {
		/// エラーの内容
		pub description: String,
	}

	impl ApplicationError {
		/// 新しいインスタンスを返します。
		pub fn new(description: &str) -> ApplicationError {
			return ApplicationError { description: description.to_string() };
		}
	}

	impl std::fmt::Display for ApplicationError {
		/// [std::fmt::Display] として振る舞うための既定の操作を提供します。
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.description)
		}
	}

	impl std::error::Error for ApplicationError {
		/// [std::error::Error] として振る舞うための既定の操作を提供します。
		///
		/// # Returns エラーの内容
		fn description(&self) -> &str {
			&self.description
		}
	}
}

#[derive(Debug, Default, serde_derive::Serialize, serde_derive::Deserialize)]
struct ApplicationRequestContent {
	message_method: Option<String>,
	content: Option<String>,
}

/// 要求をパースします。
///
/// # Arguments
/// `request_string` 要求の本文
#[allow(unused)]
fn parse_request_data_json(request_string: &str) -> std::result::Result<ApplicationRequestContent, std::boxed::Box<dyn std::error::Error>> {
	// 要求をパース
	let result = serde_json::from_str::<ApplicationRequestContent>(request_string);
	if result.is_err() {
		println!("[TRACE] {}", result.err().unwrap());
		return Err(Box::new(application_defined_errors::ApplicationError::new("無効な要求です。")));
	}
	let request_data = result.unwrap();
	return Ok(request_data);
}

/// ピアソケットからリクエストデータを読みます。
fn accept(peer_socket: &mut std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
	use std::io::Read;

	// 読み込みタイムアウトは15秒
	peer_socket.set_read_timeout(Some(std::time::Duration::from_secs(15)))?;

	// リクエストデータ
	let mut line = "".to_string();
	peer_socket.read_to_string(&mut line)?;

	// リクエストデータをパース
	let result = parse_request_data_json(&line);
	if result.is_err() {
		use std::io::Write;

		println!("[ERROR] リクエストのパースに失敗しています。(情報: {})", result.err().unwrap());
		let response = "{\"status\": \"OK\"}".as_bytes();
		peer_socket.write_all(&response)?;
		return Ok(());
	}

	let request_data = result.unwrap();

	println!("[TRACE] {:?}", request_data);

	return Ok(());
}

/// サーバーを立ち上げリッスンを開始します。
fn start_listener() -> Result<(), Box<dyn std::error::Error>> {
	// リッスン開始
	let addresses = [std::net::SocketAddr::from(([127, 0, 0, 1], 8082))];
	let listener = std::net::TcpListener::bind(&addresses[..])?;

	listener.set_nonblocking(true)?;

	let mut peer_count = 0 as u8;

	// 接続要求を待っています。
	for stream in listener.incoming() {
		if 20 <= peer_count {
			// 終了とする
			break;
		}

		peer_count += 1;

		if stream.is_err() {
			let error = stream.err().unwrap();
			println!("[ERROR] {}", error);
			std::thread::sleep(std::time::Duration::from_secs(1));
			continue;
		}

		// 接続要求を検出しました。
		let mut peer_socket = stream.unwrap();

		// ここは借用でなくともよい。
		accept(&mut peer_socket)?;

		std::thread::sleep(std::time::Duration::from_secs(5));
	}

	return Ok(());
}

/// メッセージ送信側
fn try_to_send_message() -> Result<(), Box<dyn std::error::Error>> {
	use std::io::Write;

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

fn run_application() -> Result<(), Box<dyn std::error::Error>> {
	// ========== サーバーのテスト ==========
	{
		let result = start_listener();
		if result.is_ok() {
			return Ok(());
		}
		let error = result.err().unwrap();
		let message_text = error.to_string();
		if !message_text.contains("10048") {
			println!("[ERROR] サーバーのリッスンに失敗しました。情報: {}", error);
			return Ok(());
		}
	}

	// ========== クライアントのテスト ==========
	{
		println!("[TRACE] クライアントとして起動しています...");
		try_to_send_message()?;
	}

	return Ok(());
}

/// アプリケーションのエントリーポイント
fn main() {
	// リッスン開始
	let result = run_application();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}

	println!("Ok.");
}
