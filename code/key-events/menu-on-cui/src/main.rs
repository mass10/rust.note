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

fn get_next_menuitem(current: &str, direction: &str) -> String {
	if direction == "Up" {
		if current == "B" {
			return "A".to_string();
		}
		if current == "C" {
			return "B".to_string();
		}
	} else if direction == "Down" {
		if current == "A" {
			return "B".to_string();
		}
		if current == "B" {
			return "C".to_string();
		}
	}
	return current.to_string();
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
	// use crossterm::style::Print;
	// use crossterm::terminal::{Clear, ClearType};
	use std::io::Write;

	// RAW MODE
	crossterm::terminal::enable_raw_mode()?;

	// CLS
	cls()?;

	let mut current_section = "A".to_string();

	// キーイベントループ
	loop {
		// メニューを表示
		execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
		println!("↓どうしますか？");

		if current_section == "A" {
			println!("[A] FUNCTION A: >  [ENTER]");
			println!("[B] FUNCTION B:           ");
			println!("[C] FUNCTION C:           ");
			execute!(stdout, crossterm::cursor::MoveTo(18, 1))?;
		} else if current_section == "B" {
			println!("[A] FUNCTION A:           ");
			println!("[B] FUNCTION B: >  [ENTER]");
			println!("[C] FUNCTION C:           ");
			execute!(stdout, crossterm::cursor::MoveTo(18, 2))?;
		} else if current_section == "C" {
			println!("[A] FUNCTION A:           ");
			println!("[B] FUNCTION B:           ");
			println!("[C] FUNCTION C: >  [ENTER]");
			execute!(stdout, crossterm::cursor::MoveTo(18, 3))?;
		} else {
			println!("[A] FUNCTION A: >  [ENTER]");
			println!("[B] FUNCTION B:           ");
			println!("[C] FUNCTION C:           ");
			execute!(stdout, crossterm::cursor::MoveTo(18, 1))?;
		}

		// 次のキー操作を待ちます。
		let key = crossterm::event::read()?;

		match key {
			Event::Key(KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::NONE }) => current_section = "A".to_string(),
			Event::Key(KeyEvent { code: KeyCode::Char('b'), modifiers: KeyModifiers::NONE }) => current_section = "B".to_string(),
			Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::NONE }) => current_section = "C".to_string(),
			// [Ctrl][C] to quit.
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
