use thread;
use util;

/// アプリケーション構造体
pub struct Application {
	_x: String,
}

impl Application {
	/// 新しいインスタンスを返します。
	pub fn new() -> Application {
		return Application { _x: "".to_string() };
	}

	/// アプリケーションをスタートします。
	pub fn run(&self) {
		// ========== 初期化 ==========
		println!("{} [TRACE] ({:?}) ### START ###", util::get_timestamp(), std::thread::current().id());

		// TODO Arc でくるんだオブジェクトを介して安全に情報の伝達ができるのか
		let shared_object = std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new()));

		// 通信用インターフェイスを初期化します。
		let (tx, rx) = std::sync::mpsc::channel();

		// ========== スレッドを起動 ==========

		// スレッドを起動します。
		println!("{} [TRACE] ({:?}) スレッドを起動します。", util::get_timestamp(), std::thread::current().id());
		let thread = thread::Thread::new();
		let result = thread.start(tx, &shared_object);
		if result.is_err() {
			println!("{} [ERROR] ({:?}) {}", util::get_timestamp(), std::thread::current().id(), result.err().unwrap());
			return;
		}
		let handle = result.ok().unwrap();

		// ========== メインループ ==========

		// メッセージの受信
		loop {
			println!("{} [TRACE] ({:?}) メッセージの受信を待っています。", util::get_timestamp(), std::thread::current().id());
			let result = rx.recv();
			if result.is_err() {
				println!(
					"{} [ERROR] ({:?}) スレッドメッセージの受信に失敗しました。理由: [{}]",
					util::get_timestamp(),
					std::thread::current().id(),
					result.err().unwrap()
				);
				break;
			}
			let result = result.unwrap();
			println!(
				"{} [TRACE] ({:?}) スレッドからのメッセージ: [{}]",
				util::get_timestamp(),
				std::thread::current().id(),
				result
			);
			std::thread::sleep(std::time::Duration::from_millis(1));
		}

		// ========== 待機 ==========

		// スレッド終了まで待機します。
		println!("{} [TRACE] ({:?}) スレッド終了まで待機しています...", util::get_timestamp(), std::thread::current().id());
		let result = handle.join();
		if result.is_err() {
			println!("{} [ERROR] ({:?}) {:?}", util::get_timestamp(), std::thread::current().id(), result.err().unwrap());
			return;
		}

		// スレッドの応答
		let thread_response = result.unwrap();
		println!(
			"{} [TRACE] ({:?}) スレッドの応答: [{}]",
			util::get_timestamp(),
			std::thread::current().id(),
			thread_response
		);

		// ========== 終了 ==========

		println!("{} [TRACE] ({:?}) --- END ---", util::get_timestamp(), std::thread::current().id());
	}
}
