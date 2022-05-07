//! スレッド関連

use crate::debug;
use crate::error;
use crate::util;

/// スレッドの実装
pub struct Thread01 {}

impl Thread01 {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// `Thread`
	pub fn new() -> Thread01 {
		return Thread01 {};
	}

	/// スレッド内の何かの処理
	///
	/// # Parameters
	/// * `success_count` - 現在の成功カウンター
	/// * `tx` - メッセージ送信用のチャネル
	/// # Returns
	/// 処理結果
	fn try_and_respond(success_count: i32, tx: &std::sync::mpsc::Sender<String>) -> Result<bool, Box<dyn std::error::Error>> {
		// タイムスタンプ
		let timestamp = util::get_current_timestamp();
		if !timestamp.ends_with("0") {
			// 何もしない
			return Ok(false);
		}

		// スレッドからのメッセージをメインスレッドへ送信します。
		let thread_message = format!("{} スレッドからのメッセージ({})", timestamp, success_count);

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
				let result = Self::try_and_respond(success_count, &tx);
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
				std::thread::sleep(std::time::Duration::from_millis(75));
			}

			// レスポンス
			debug!("--- exit thread ---");
			return "スレッドの応答".to_string();
		});

		debug!("スレッドは正常に実行されました。");

		return Ok(handle);
	}
}
