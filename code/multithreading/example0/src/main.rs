extern crate chrono;

/// タイムスタンプを生成します。
pub fn get_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// スレッドを起動します。
fn start_thread(tx: std::sync::mpsc::Sender<String>, _map: &std::sync::Arc<std::sync::Mutex<std::collections::BTreeMap<String, String>>>) -> std::result::Result<std::thread::JoinHandle<String>, Box<dyn std::error::Error>> {
	let handle = std::thread::spawn(move || {
		println!("{} [TRACE] ({:?}) $$$ begin thread $$$", get_timestamp(), std::thread::current().id());

		loop {
			// TODO Arc でくるんだオブジェクトを介して安全に情報の伝達ができるのか
			// map.lock();
			// map.as_ref().set

			// タイムスタンプ
			let timestamp = get_timestamp();
			// ある条件が成立するとスレッドは終了します。
			let exit_condition = if timestamp.ends_with("0") { true } else { false };

			// スレッドからのメッセージをメインスレッドへ送信します。
			let thread_message = format!("{} スレッドからのメッセージ{}", timestamp, if exit_condition { "(終了)" } else { "" });
			let result = tx.send(thread_message);
			if result.is_err() {
				let error = result.err().unwrap();
				println!("{} [ERROR] ({:?}) Unknown error. reason: [{}]", get_timestamp(), std::thread::current().id(), error);
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
		println!("{} [TRACE] ({:?}) --- exit thread ---", get_timestamp(), std::thread::current().id());
		return response.to_string();
	});

	println!("{} [TRACE] ({:?}) スレッドは正常に実行されました。", get_timestamp(), std::thread::current().id());

	return Ok(handle);
}

fn main() {
	println!("{} [TRACE] ({:?}) ### START ###", get_timestamp(), std::thread::current().id());

	// TODO Arc でくるんだオブジェクトを介して安全に情報の伝達ができるのか
	let shared_object: std::sync::Arc<std::sync::Mutex<std::collections::BTreeMap<String, String>>> = std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new()));

	// 通信用インターフェイスを初期化します。
	let (tx, rx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();

	// スレッドを起動します。
	println!("{} [TRACE] ({:?}) スレッドを起動します。", get_timestamp(), std::thread::current().id());
	let result = start_thread(tx, &shared_object);
	if result.is_err() {
		println!("{} [ERROR] ({:?}) {}", get_timestamp(), std::thread::current().id(), result.err().unwrap());
		return;
	}
	let handle = result.ok().unwrap();

	// メッセージの受信
	loop {
		println!("{} [TRACE] ({:?}) メッセージの受信を待っています。", get_timestamp(), std::thread::current().id());
		let result = rx.recv();
		if result.is_err() {
			println!("{} [ERROR] ({:?}) スレッドメッセージの受信に失敗しました。理由: [{}]", get_timestamp(), std::thread::current().id(), result.err().unwrap());
			break;
		}
		let result = result.unwrap();
		println!("{} [TRACE] ({:?}) スレッドからのメッセージ: [{}]", get_timestamp(), std::thread::current().id(), result);
		std::thread::sleep(std::time::Duration::from_millis(1));
	}

	// スレッド終了まで待機します。
	println!("{} [TRACE] ({:?}) スレッド終了まで待機しています...", get_timestamp(), std::thread::current().id());
	let result = handle.join();
	if result.is_err() {
		println!("{} [ERROR] ({:?}) {:?}", get_timestamp(), std::thread::current().id(), result.err().unwrap());
		return;
	}

	// スレッドの応答
	let thread_response = result.unwrap();
	println!("{} [TRACE] ({:?}) スレッドの応答: [{}]", get_timestamp(), std::thread::current().id(), thread_response);

	println!("{} [TRACE] ({:?}) --- END ---", get_timestamp(), std::thread::current().id());
}
