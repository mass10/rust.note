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
		let thread = thread::Thread::new();
		let result = thread.start(tx, &shared_object);
		if result.is_err() {
			error!("{}", result.err().unwrap());
			return Ok(());
		}
		let handle = result.ok().unwrap();

		// ========== メインループ ==========

		// メッセージの受信
		loop {
			debug!("メッセージの受信を待っています。",);
			let result = rx.recv();
			if result.is_err() {
				// TODO: 失敗でない穏やかな終了方法
				error!("スレッドメッセージの受信に失敗しました。理由: [{}]", result.err().unwrap());
				break;
			}

			let message = result.unwrap();

			debug!("スレッドからのメッセージ: [{}]", message);

			std::thread::sleep(std::time::Duration::from_millis(1));
		}

		// ========== 待機 ==========

		// スレッド終了まで待機します。
		debug!("スレッド終了まで待機しています...",);
		let result = handle.join();
		if result.is_err() {
			error!("{:?}", result.err().unwrap());
			return Ok(());
		}

		// スレッドの応答
		let thread_response = result.unwrap();
		debug!("スレッドの応答: [{}]", thread_response);

		// ========== 終了 ==========

		debug!("--- END ---");

		return Ok(());
	}
}
