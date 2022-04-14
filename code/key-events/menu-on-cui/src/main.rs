//!
//! CLI ツールでメニューの選択を提供するサンプルです。
//!

#[macro_use]
extern crate crossterm;

/// 画面を消去します。
#[allow(unused)]
fn cls() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	use crossterm::terminal::{Clear, ClearType};
	// use std::io::Write;

	let mut stdout = std::io::stdout();
	// 画面を消去します。
	execute!(stdout, Clear(ClearType::All))?;
	// 左上 [0, 0] へ移動します。
	execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
	return Ok(());
}

/// メニューを表示します。
struct MyMenuController {
	/// メニューの表示項目を保持します。
	menuitems: std::collections::BTreeMap<String, String>,
	/// 現在選択中のメニュー項目を保持します。
	_current_section: String,
}

impl MyMenuController {
	/// 新しいインスタンスを返します。
	pub fn new() -> Self {
		return Self {
			menuitems: std::collections::BTreeMap::new(),
			_current_section: String::from(""),
		};
	}

	/// メニューアイテムを追加します。
	///
	/// # Arguments
	/// * `key` - 内部キー
	/// * `description` - メニューアイテム
	pub fn add_menuitem(&mut self, key: &str, description: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		self.menuitems.insert(key.to_string(), description.to_string());
		return Ok(());
	}

	/// メニューを表示します。
	///
	/// # Arguments
	/// * `current_section` - 現在の選択項目
	pub fn show_menuitems(&self, current_section: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		let mut stdout = std::io::stdout();

		if current_section == "A" {
			println!("[*] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[ ] EXIT");

			let (_, y) = crossterm::cursor::position()?;
			execute!(stdout, crossterm::cursor::MoveTo(1, y - 3))?;
		} else if current_section == "B" {
			println!("[ ] FUNCTION A");
			println!("[*] FUNCTION B");
			println!("[ ] EXIT");

			let (_, y) = crossterm::cursor::position()?;
			execute!(stdout, crossterm::cursor::MoveTo(1, y - 2))?;
		} else if current_section == "X" {
			println!("[ ] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[*] EXIT");

			let (_, y) = crossterm::cursor::position()?;
			execute!(stdout, crossterm::cursor::MoveTo(1, y - 1))?;
		} else {
			println!("[*] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[ ] EXIT");

			let (_, y) = crossterm::cursor::position()?;
			execute!(stdout, crossterm::cursor::MoveTo(1, y - 3))?;
		}

		return Ok(());
	}

	/// メニューを表示します。
	pub fn show(&mut self) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
		use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
		// use crossterm::style::Print;
		// use crossterm::terminal::{Clear, ClearType};
		// use std::io::Write;
		// let (_, y) = crossterm::cursor::position()?;

		// デフォルトポジション
		let mut current_section = "A".to_string();

		let mut prev_current_section = "".to_string();

		// キーイベントループ
		loop {
			// メニューを表示
			// execute!(stdout, crossterm::cursor::MoveTo(0, y))?;

			// メニュー項目を表示します。
			reset_cursor_position(&prev_current_section)?;
			self.show_menuitems(&current_section)?;

			prev_current_section = current_section.clone();

			// 次のキー操作を待ちます。
			let key = crossterm::event::read()?;

			match key {
				// [A]
				Event::Key(KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::NONE }) => current_section = "A".to_string(),
				// [B]
				Event::Key(KeyEvent { code: KeyCode::Char('b'), modifiers: KeyModifiers::NONE }) => current_section = "B".to_string(),
				// [X]
				Event::Key(KeyEvent { code: KeyCode::Char('x'), modifiers: KeyModifiers::NONE }) => current_section = "X".to_string(),
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

		return Ok(current_section);
	}
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
		if current == "X" {
			return "B".to_string();
		}
		return "X".to_string();
	} else if direction == "Down" {
		if current == "A" {
			return "B".to_string();
		}
		if current == "B" {
			return "X".to_string();
		}
		return "A".to_string();
	}
	return current.to_string();
}

#[allow(unused)]
fn move_y(amount: i16) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	let (x, mut y) = crossterm::cursor::position()?;
	if amount > 0 {
		y = y + amount as u16;
	} else {
		y = y.saturating_sub(amount.abs() as u16);
	}
	execute!(stdout, crossterm::cursor::MoveTo(x, y))?;
	return Ok(());
}

fn reset_cursor_position(prev_current_section: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();

	if prev_current_section == "A" {
		let (_, y) = crossterm::cursor::position()?;
		execute!(stdout, crossterm::cursor::MoveTo(0, y))?;
	} else if prev_current_section == "B" {
		let (_, y) = crossterm::cursor::position()?;
		execute!(stdout, crossterm::cursor::MoveTo(0, y - 1))?;
	} else if prev_current_section == "X" {
		let (_, y) = crossterm::cursor::position()?;
		execute!(stdout, crossterm::cursor::MoveTo(0, y - 2))?;
	}
	return Ok(());
}

/// メニュー項目を表示してユーザーに選択させるサンプルです。
fn run() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	// RAW MODE
	crossterm::terminal::enable_raw_mode()?;

	// CLS
	if false {
		cls()?;
	}

	println!("↓どうしますか？");

	// メニューを構築
	let mut menu = MyMenuController::new();
	menu.add_menuitem("A", "FUNCTION A")?;
	menu.add_menuitem("B", "FUNCTION B")?;
	menu.add_menuitem("X", "EXIT")?;

	// メニューを表示してユーザー入力を得ます。
	let result = menu.show()?;

	// TODO: 表示される位置を調整する
	// execute!(stdout, crossterm::cursor::MoveTo(0, y + 4))?;

	println!();
	println!("[{}] が選択されました。", result);

	return Ok(());
}

/// アプリケーションのエントリーポイントです。
fn main() {
	let result = run();
	if result.is_err() {
		println!("{}", result.unwrap_err());
		return;
	}
}
