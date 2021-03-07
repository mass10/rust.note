use super::client;
use super::listener;

fn run_server() -> Result<(), Box<dyn std::error::Error>> {
	let result = std::process::Command::new("simple-tcp-app.exe").arg("--server").spawn();
	if result.is_err() {
		println!("[ERROR] Cannot create a process. reason: {}", result.err().unwrap());
		return Ok(());
	}
	let mut command = result.unwrap();
	if false {
		let result = command.wait();
		if result.is_err() {
			println!("[ERROR] Cannot create a process. reason: {}", result.err().unwrap());
			return Ok(());
		}
	}
	return Ok(());
}

///
/// アプリケーション本体
///
pub struct Application;

impl Application {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// 新しいインスタンス
	pub fn new() -> Application {
		return Application {};
	}

	/// アプリケーションを実行します。
	pub fn start(&self, request: &str) -> Result<(), Box<dyn std::error::Error>> {
		// ========== サーバーモードで起動 ==========
		if request == "--server" {
			listener::start_listener();
			return Ok(());
		}

		// ========== サーバープロセスの起動 ==========
		run_server()?;

		// ========== クライアントの実装 ==========
		client::try_to_send_message()?;

		return Ok(());
	}
}
