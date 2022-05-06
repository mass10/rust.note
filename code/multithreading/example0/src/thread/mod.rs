//! スレッド関連

use crate::debug;
use crate::error;
use crate::util;

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

	/// スレッド内の何かの処理
	///
	/// # Parameters
	/// * `tx` - メッセージ送信用のチャネル
	/// # Returns
	/// 処理結果
	fn try_respond(tx: &std::sync::mpsc::Sender<String>) -> Result<bool, Box<dyn std::error::Error>> {
		// タイムスタンプ
		let timestamp = util::get_current_timestamp();
		if !timestamp.ends_with("0") {
			// 何もしない
			return Ok(false);
		}

		// スレッドからのメッセージをメインスレッドへ送信します。
		let thread_message = format!("{} スレッドからのメッセージ", timestamp);
		tx.send(thread_message)?;

		return Ok(true);
	}

	/// スレッドを起動します。
	pub fn start(
		&self,
		tx: std::sync::mpsc::Sender<String>,
		_map: &std::sync::Arc<std::sync::Mutex<std::collections::BTreeMap<String, String>>>,
	) -> std::result::Result<std::thread::JoinHandle<String>, Box<dyn std::error::Error>> {
		let handle = std::thread::spawn(move || {
			debug!("$$$ begin thread $$$");

			// 成功カウンター(スレッドの終了条件)
			let mut success_count = 0;

			loop {
				// TODO Arc でくるんだオブジェクトを介して安全に情報の伝達ができるのか
				// map.lock();
				// map.as_ref().set

				// ================ 終了条件 ================
				// ある条件が成立するとスレッドは終了します。
				if 10 <= success_count {
					break;
				}

				// ================ 一定期間で実施する何らかの処理 ================
				// ※このスコープでメソッドを呼び出すことはできない。
				let result = Self::try_respond(&tx);
				if result.is_err() {
					let error = result.err().unwrap();
					error!("Unknown error. reason: [{}]", error);
					// 復旧不能とみなす
					break;
				}

				// 結果の確認
				let status = result.unwrap();
				if status {
					success_count += 1;
				}

				// 少し待機
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
