//!
//! CLI ツールでメニューの選択を提供するサンプルです。
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

/// メニューアイテムを移動します。
///
/// # Arguments 
/// * `current` - 基準となる(=現在の)メニューアイテム
/// * `direction` - 移動方向
fn get_next_menuitem(current: &str, direction: &str) -> String {
	if direction == "Up" {
		if current == "B" {
			return "A".to_string();
		}
		if current == "C" {
			return "B".to_string();
		}
		return "C".to_string();
	} else if direction == "Down" {
		if current == "A" {
			return "B".to_string();
		}
		if current == "B" {
			return "C".to_string();
		}
		return "A".to_string();
	}
	return current.to_string();
}

/// メニュー項目を表示してユーザーに選択させるサンプルです。
fn test_show_menuitems() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
	// use crossterm::style::Print;
	// use crossterm::terminal::{Clear, ClearType};
	use std::io::Write;

	// RAW MODE
	crossterm::terminal::enable_raw_mode()?;

	// CLS
	cls()?;

	let mut current_section = "".to_string();

	// キーイベントループ
	loop {
		// メニューを表示
		execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;

		println!("↓どうしますか？");

		if current_section == "A" {
			println!("[*] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[ ] FUNCTION C");

			execute!(stdout, crossterm::cursor::MoveTo(1, 1))?;
		} else if current_section == "B" {
			println!("[ ] FUNCTION A");
			println!("[*] FUNCTION B");
			println!("[ ] FUNCTION C");

			execute!(stdout, crossterm::cursor::MoveTo(1, 2))?;
		} else if current_section == "C" {
			println!("[ ] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[*] FUNCTION C");

			execute!(stdout, crossterm::cursor::MoveTo(1, 3))?;
		} else {
			println!("[*] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[ ] FUNCTION C");

			execute!(stdout, crossterm::cursor::MoveTo(1, 1))?;
		}

		// 次のキー操作を待ちます。
		let key = crossterm::event::read()?;

		match key {
			// [A]
			Event::Key(KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::NONE }) => current_section = "A".to_string(),
			// [B]
			Event::Key(KeyEvent { code: KeyCode::Char('b'), modifiers: KeyModifiers::NONE }) => current_section = "B".to_string(),
			// [C]
			Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::NONE }) => current_section = "C".to_string(),
			// [Ctrl] + [C] to quit.
			Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => break,
			// [Q] to quit.
			Event::Key(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE }) => break,
			// [Up]
			Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE }) => {
				current_section = get_next_menuitem(&current_section, "Up");
				// let mut stdout = std::io::stdout();
				// execute!(stdout, crossterm::cursor::MoveTo(16, 1))?;
			}
			// [Down]
			Event::Key(KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE }) => {
				current_section = get_next_menuitem(&current_section, "Down");
				// let mut stdout = std::io::stdout();
				// execute!(stdout, crossterm::cursor::MoveTo(16, 2))?;
			}
			Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => break,
			// Else
			_ => {
				// 画面を消去します。
				// cls()?;
				// キーを表示します。
				// print!("> KEY: {:?}", e);
				// std::io::stdout().flush()?;
			}
		}
	}

	execute!(stdout, crossterm::cursor::MoveTo(0, 4))?;

	println!("選択されたのは {}", current_section);

	return Ok(());
}

/// アプリケーションのエントリーポイントです。
fn main() {
	let result = test_show_menuitems();
	if result.is_err() {
		println!("{}", result.unwrap_err());
		return;
	}
}
