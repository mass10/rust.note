use crate::{debug, error, info};

pub struct Thread1 {
	/// メッセージ送信用のチャンネル
	sender: Option<std::sync::mpsc::Sender<String>>,
	// marker: std::marker::PhantomData<()>,
	/// ハンドル
	handle: Option<std::thread::JoinHandle<()>>,
}

impl Thread1 {
	/// 新しいインスタンスを返します。
	///
	/// * 同時に内部スレッドが開始されます。
	pub fn new() -> Self {
		let mut instance = Thread1 {
			sender: None,
			handle: None,
		};
		instance.run();
		return instance;
	}

	/// 内部スレッドを終了します。
	pub fn terminate(&mut self) {
		// 停止要求
		self.request("terminate");

		// 待機
		let result = self.handle.take();
		if result.is_none() {
			return;
		}
		let handle = result.unwrap();
		handle.join().unwrap();

		// チャネルをクローズ
		self.sender = None;
	}

	/// 内部スレッドにメッセージを送信します。
	pub fn request(&mut self, msg: &str) {
		if self.sender.is_none() {
			return;
		}
		let sender = self.sender.as_ref().unwrap();
		let result = sender.send(msg.to_string());
		if result.is_err() {
			let error = result.err().unwrap();
			error!("{}", error);
		}
	}

	/// 内部スレッドを実行します。
	fn run(&mut self) {
		let (sender, receiver) = std::sync::mpsc::channel();
		self.sender = Some(sender);

		let thread = std::thread::spawn(move || {
			info!("$$$ Start $$$");

			// ループ開始
			loop {
				// チャネルを確認
				let result = receiver.try_recv();
				if result.is_err() {
					let reason = result.err().unwrap();
					if reason == std::sync::mpsc::TryRecvError::Empty {
						// 何もしない
					} else if reason == std::sync::mpsc::TryRecvError::Disconnected {
						break;
					}
					std::thread::sleep(std::time::Duration::from_millis(50));
					continue;
				}

				// 受信
				let msg = result.unwrap();
				if msg == "terminate" {
					info!("スレッドを終了します。");
					break;
				}
				debug!("{}", msg);

				std::thread::sleep(std::time::Duration::from_millis(50));
			}

			info!("--- End ---");
		});

		info!("スレッドを開始しました。");

		self.handle = Some(thread);
	}
}

impl Drop for Thread1 {
	fn drop(&mut self) {
		self.terminate();
	}
}
