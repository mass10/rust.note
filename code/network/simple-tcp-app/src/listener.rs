use super::application_defined_errors;

#[derive(Debug, Default, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApplicationRequestContent {
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
pub fn accept(mut peer_socket: std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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

/// リスナーを起動します。
pub fn start_listener() {
	use super::util;

	// リスナーを起動します。
	let addresses = [std::net::SocketAddr::from(([127, 0, 0, 1], 8082))];
	let result = std::net::TcpListener::bind(&addresses[..]);
	if result.is_err() {
		let error = result.err().unwrap();
		let message_text = error.to_string();
		if message_text.contains("10048") {
			// Address already in use
			return;
		}
		// その他の問題
		return;
	}

	let listener = result.unwrap();

	let result = listener.set_nonblocking(true);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
	}

	println!("[TRACE] $$$ BEGIN LISTENER $$$");

	// タイムキーパー
	let time_keeper = util::TimeKeeper::new();

	// 接続要求を待っています。
	for stream in listener.incoming() {
		if time_keeper.is_over() {
			break;
		}
		if stream.is_ok() {
			// 接続要求を検出しました。
			let result = accept(stream.unwrap());
			if result.is_err() {
				println!("[ERROR] {}", result.err().unwrap());
			}
			std::thread::sleep(std::time::Duration::from_millis(5));
		} else if stream.is_err() {
			let e = stream.err().unwrap();
			if e.kind() == std::io::ErrorKind::WouldBlock {
				std::thread::sleep(std::time::Duration::from_millis(5));
			} else {
				println!("[ERROR] {}", e);
				break;
			}
		}
	}

	println!("[TRACE] --- EXIT LISTENER ---");
}
