use std::fs::remove_file;

fn execute_spawn() -> Result<(), Box<dyn std::error::Error>> {
	let result = std::process::Command::new("spawning-process.exe").arg("--daemon").spawn();
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

/// 停止要求の確認
fn exists_stop_request() -> bool {
	let shutdown_file = ".shutdown";

	if !std::path::Path::new(shutdown_file).exists() {
		return false;
	}

	let result = remove_file(shutdown_file);
	if result.is_err() {
		return false;
	}

	return true;
}

struct TimeKeeper {
	start_time: std::time::Instant,
}

impl TimeKeeper {
	pub fn new() -> TimeKeeper {
		return TimeKeeper {
			start_time: std::time::Instant::now(),
		};
	}

	pub fn is_over(&self) -> bool {
		let current_time = std::time::Instant::now();
		let elapsed = current_time - self.start_time;
		return 10 <= elapsed.as_secs();
	}
}

/// 続行モードで実行。
fn execute_daemon() -> Result<(), Box<dyn std::error::Error>> {
	let time_keeper = TimeKeeper::new();

	loop {
		// タイムキーパーによる終了確認
		if time_keeper.is_over() {
			// 終了と言っている。
			break;
		}

		// プロセスの停止要求を確認します。
		if exists_stop_request() {
			break;
		}

		// 少し待機
		std::thread::sleep(std::time::Duration::from_secs(1));
	}

	return Ok(());
}

#[allow(unused)]
fn usage() {
	println!("USAGE:");
	println!("    --contiuous: Run contiuous process.");
	println!("    --spawn: Run main process.");
}

fn execute(request: &str) -> Result<(), Box<dyn std::error::Error>> {
	match request {
		"--daemon" => execute_daemon()?,
		_ => execute_spawn()?,
	}

	return Ok(());
}

/// Application entrypoint.
fn main() {
	// Commandline argument
	let args: Vec<String> = std::env::args().skip(1).collect();
	let request = if 0 < args.len() { &args[0] } else { "" };

	// Execute.
	let result = execute(request);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
