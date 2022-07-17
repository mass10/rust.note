use crate::debug;
use crate::error;
use crate::thread;
use crate::util;

/// アプリケーション構造体
pub struct Application {}

impl Application {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// 新しいインスタンス
	pub fn new() -> Application {
		return Application {};
	}

	/// アプリケーションをスタートします。
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		// ========== 初期化 ==========
		debug!("### START ###");

		// TODO Arc でくるんだオブジェクトを介して安全に情報の伝達ができるのか
		let shared_object = std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new()));

		// 通信用インターフェイスを初期化します。
		let (tx, rx) = std::sync::mpsc::channel();

		// ========== スレッドを起動 ==========

		// スレッドを起動します。
		debug!("スレッドを起動します。");
		let mut thread = thread::Thread01::new();
		let result = thread.start(tx, &shared_object);
		if result.is_err() {
			error!("{}", result.err().unwrap());
			return Ok(());
		}

		// ========== メインループ ==========

		// メッセージの受信
		// TODO: 受信ループを別のスレッドにする
		loop {
			debug!("メッセージの受信を待っています。",);
			let result = rx.recv();
			if result.is_err() {
				// TODO: 失敗(receiving on a closed channel)ではなく、確かな終了ステータスで終了させる
				error!("スレッドメッセージの受信に失敗しました。理由: [{}]", result.err().unwrap());
				break;
			}

			let message = result.unwrap();

			debug!("スレッドからのメッセージ: [{}]", message);

			std::thread::sleep(std::time::Duration::from_millis(1));
		}

		// ========== 待機 ==========
		debug!("スレッド終了まで待機しています...",);
		let result = thread.join();
		if result.is_err() {
			let error = result.err().unwrap();
			error!("スレッドは正常に終了しませんでした。エラー: {:?}", error);
		} else {
			let thread_response = result.unwrap();
			debug!("スレッドの応答: [{}]", thread_response);
		}

		// ========== 終了 ==========
		debug!("--- END ---");

		return Ok(());
	}
}
