use crate::info;

use crate::thread1;
use crate::thread2;

/// アプリケーション構造体
pub struct Application {}

impl Application {
	/// 新しいインスタンスを返します。
	pub fn new() -> Self {
		Application {}
	}

    /// アプリケーションを実行します。
	pub fn run(&self) {
		info!("### Start ###");

		// 何らかの非同期処理を実施するタスク(1)
		let mut thread1 = thread1::Thread1::new();
		// 何らかの非同期処理を実施するタスク(2)
		let _thread2 = thread2::Thread2::new();

		let clock = std::time::Instant::now();

		loop {
			// 一定時間が経過したら終了
			if 20 < clock.elapsed().as_secs() {
				info!("Time's up!");
				break;
			}

			thread1.request("Hello");

			// 少しだけ待機
			std::thread::sleep(std::time::Duration::from_millis(1000 * 3));
		}

        thread1.terminate();

		info!("--- End ---");
	}
}
