#[macro_use]
extern crate crossterm;

#[allow(unused)]
use std::io::Write;

use crossterm::event::KeyEvent;

/// 画面を消去します。
#[allow(unused)]
fn cls() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	use crossterm::terminal::{Clear, ClearType};

	let mut stdout = std::io::stdout();
	// 画面を消去します。
	execute!(stdout, Clear(ClearType::All))?;
	// 左上 [0, 0] へ移動します。
	execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
	return Ok(());
}

///
/// 簡易タイムキーパー
///
struct TimeKeeper {
	start: Option<std::time::Instant>,
	times: u32,
}

impl TimeKeeper {
	/// 新しいインスタンスを生成します。
	///
	/// # Returns
	/// 新しいインスタンス
	pub fn new() -> TimeKeeper {
		return TimeKeeper { start: None, times: 0 };
	}

	/// 計測を開始します。
	pub fn start(&mut self) {
		self.start = Some(std::time::Instant::now());
	}

	/// 計測を開始したか確認します。
	///
	/// #
	fn started(&mut self) -> bool {
		return self.start.is_some();
	}

	pub fn checkout(&mut self) -> bool {
		if !self.started() {
			// Not started.
			return false;
		}

		self.times += 1;
		let current_time = std::time::Instant::now();
		let erapsed = current_time - self.start.unwrap();
		if erapsed.as_millis() < 400 {
			return false;
		}
		return true;
	}
}

#[allow(unused)]
fn countdown() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	crossterm::execute!(stdout, crossterm::cursor::SavePosition)?;
	for n in 1..4 {
		crossterm::execute!(stdout, crossterm::cursor::RestorePosition)?;
		print!("({})", n);
		std::io::stdout().flush()?;
		std::thread::sleep(std::time::Duration::from_millis(1000));
	}
	return Ok(());
}

fn detect_key() -> std::result::Result<Option<crossterm::event::Event>, std::boxed::Box<dyn std::error::Error>> {
	let result = crossterm::event::poll(std::time::Duration::from_millis(10))?;
	if !result {
		return Ok(None);
	}
	// 次のキー操作を待ちます。
	let key = crossterm::event::read()?;
	return Ok(Some(key));
}

fn run_app() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	// use crossterm::style::Print;
	use crossterm::event::{Event, KeyCode, KeyModifiers};
	// use crossterm::terminal::{Clear, ClearType};

	let mut stdout = std::io::stdout();

	// MAKE APPLICATION RAW MODE
	crossterm::terminal::enable_raw_mode()?;

	// SAVE CURSOR POSITION
	crossterm::execute!(stdout, crossterm::cursor::SavePosition)?;
	// crossterm::execute!(stdout, crossterm::cursor::RestorePosition)?;

	// CLS
	// cls()?;

	let mut current_complexity = 0;

	//
	// COUNTDOWN
	//
	if false {
		countdown()?;
	}

	//
	// BEGIN
	//
	{
		crossterm::execute!(stdout, crossterm::cursor::RestorePosition)?;
		print!("[Enter] e");
		std::io::stdout().flush()?;
	}

	let mut time_keeper = TimeKeeper::new();

	// キーイベントループ
	loop {
		// 終了判断
		if time_keeper.checkout() {
			break;
		}

		// 次のキー操作を待ちます。
		let key = detect_key()?;
		if key.is_none() {
			continue;
		}

		time_keeper.start();

		match key.unwrap() {
			// [Ctrl][C] to quit.
			Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => break,
			// [Enter]
			Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
				current_complexity = current_complexity + 1;
				print!("e");
				std::io::stdout().flush()?;
			}
			// [Enter]
			Event::Key(KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE }) => {
				current_complexity = current_complexity + 1;
				print!("e");
				std::io::stdout().flush()?;
			}
			// Else
			_ => break,
		}
	}

	println!();
	println!("complexity: {}", current_complexity);

	return Ok(());
}

fn main() {
	let result = run_app();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] {}", error);
		return;
	}
}
