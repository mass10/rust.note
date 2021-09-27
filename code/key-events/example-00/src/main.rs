//!
//! キーイベントを受け取って処理をする単純なアプリケーションです。
//!

#[macro_use]
extern crate crossterm;

/// 画面を消去します。
fn cls() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	use crossterm::terminal::{Clear, ClearType};
	use std::io::Write;

	let mut stdout = std::io::stdout();
	// 画面を消去します。
	execute!(stdout, Clear(ClearType::All))?;
	// 左上 [0, 0] へ移動します。
	execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
	return Ok(());
}

/// キーイベント
///
/// # Arguments
/// * `key` - キーイベント
fn on_key_down(key: crossterm::event::Event) -> std::result::Result<bool, std::boxed::Box<dyn std::error::Error>> {
	use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
	use crossterm::style::Print;
	use crossterm::terminal::{Clear, ClearType};
	use std::io::Write;

	match key {
		Event::Key(KeyEvent { code: KeyCode::Char('l'), modifiers: KeyModifiers::CONTROL }) => {
			execute!(std::io::stdout(), Clear(ClearType::All), Print(""))?;
			return Ok(true);
		}
		Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::ALT }) => return Ok(false),
		// [Ctrl][C] to quit.
		Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => return Ok(false),
		// [Q] to quit.
		Event::Key(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE }) => return Ok(false),
		// Else
		e => {
			// 画面を消去します。
			cls()?;
			// キーを表示します。
			print!("> KEY: {:?}", e);
			std::io::stdout().flush()?;
			return Ok(true);
		}
	}
}

/// アプリケーションのエントリーポイント
fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	// RAW MODE
	crossterm::terminal::enable_raw_mode()?;

	// CLS
	cls()?;

	// キーイベントループ
	loop {
		// 次のキー操作を待ちます。
		let key = crossterm::event::read()?;
		if !on_key_down(key)? {
			break;
		}
	}

	return Ok(());
}
