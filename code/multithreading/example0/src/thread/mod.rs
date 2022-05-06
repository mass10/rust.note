//! スレッド関連

use crate::debug;
use util;

/// スレッドの実装
pub struct Thread {
	_x: String,
}

impl Thread {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// `Thread`
	pub fn new() -> Thread {
		return Thread { _x: "".to_string() };
	}

	/// スレッドを起動します。
	pub fn start(
		&self,
		tx: std::sync::mpsc::Sender<String>,
		_map: &std::sync::Arc<std::sync::Mutex<std::collections::BTreeMap<String, String>>>,
	) -> std::result::Result<std::thread::JoinHandle<String>, Box<dyn std::error::Error>> {
		let handle = std::thread::spawn(move || {
			debug!("$$$ begin thread $$$");

			loop {
				// TODO Arc でくるんだオブジェクトを介して安全に情報の伝達ができるのか
				// map.lock();
				// map.as_ref().set

				// タイムスタンプ
				let timestamp = util::get_current_timestamp();
				// ある条件が成立するとスレッドは終了します。
				let exit_condition = if timestamp.ends_with("0") { true } else { false };

				// スレッドからのメッセージをメインスレッドへ送信します。
				let thread_message = format!("{} スレッドからのメッセージ{}", timestamp, if exit_condition { "(終了)" } else { "" });
				let result = tx.send(thread_message);
				if result.is_err() {
					let error = result.err().unwrap();
					println!(
						"{} [ERROR] ({:?}) Unknown error. reason: [{}]",
						util::get_current_timestamp(),
						std::thread::current().id(),
						error
					);
					// 復旧不能とみなす
					break;
				}

				// ある条件が成立するとスレッドは終了します。
				if exit_condition {
					break;
				}

				std::thread::sleep(std::time::Duration::from_millis(234));
			}

			// レスポンス
			let response = "スレッドの応答";
			debug!("--- exit thread ---");
			return response.to_string();
		});

		debug!("スレッドは正常に実行されました。");

		return Ok(handle);
	}
}
