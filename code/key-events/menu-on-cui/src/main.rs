//!
//! CLI ツールでメニューの選択を提供するサンプルです。
//!

use crossterm::cursor;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;

#[macro_use]
extern crate crossterm;

/// 画面を消去します。
#[allow(unused)]
fn cls() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	// 画面を消去します。
	execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
	// 左上 [0, 0] へ移動します。
	execute!(stdout, cursor::MoveTo(0, 0))?;
	return Ok(());
}

type MenuItem = (String, KeyCode);

/// メニューを表示します。
struct MyMenuController {
	/// メニューの表示項目を保持します。
	menu_items: std::collections::BTreeMap<String, MenuItem>,
	/// 現在選択中のメニュー項目を保持します。
	#[allow(unused)]
	current_section: String,
}

impl MyMenuController {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// * `MyMenuController`
	pub fn new() -> Self {
		return Self {
			menu_items: std::collections::BTreeMap::new(),
			current_section: String::from(""),
		};
	}

	/// メニューアイテムを追加します。
	///
	/// # Arguments
	/// * `id` - 内部ID
	/// * `description` - メニューアイテム
	/// * `key_code` - メニューアイテムに関連付けられるショートカットキー
	pub fn add_menuitem(&mut self, id: &str, description: &str, key_code: KeyCode) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		let menu_item = (description.to_string(), key_code);
		self.menu_items.insert(id.to_string(), menu_item);
		return Ok(());
	}

	/// メニューを表示します。
	///
	/// # Arguments
	/// * `current_section` - 現在の選択項目
	pub fn show_menuitems(&self, current_section: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		let mut stdout = std::io::stdout();

		// TODO: メニューアイテムの正しい判定
		if current_section == "A" {
			println!("[*] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[ ] EXIT");

			let (_, y) = cursor::position()?;
			execute!(stdout, cursor::MoveTo(1, y - 3))?;
		} else if current_section == "B" {
			println!("[ ] FUNCTION A");
			println!("[*] FUNCTION B");
			println!("[ ] EXIT");

			let (_, y) = cursor::position()?;
			execute!(stdout, cursor::MoveTo(1, y - 2))?;
		} else if current_section == "X" {
			println!("[ ] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[*] EXIT");

			let (_, y) = cursor::position()?;
			execute!(stdout, cursor::MoveTo(1, y - 1))?;
		} else {
			println!("[*] FUNCTION A");
			println!("[ ] FUNCTION B");
			println!("[ ] EXIT");

			let (_, y) = cursor::position()?;
			execute!(stdout, cursor::MoveTo(1, y - 3))?;
		}

		return Ok(());
	}

	/// 指定されたキーコードでメニュー項目を検索します。
	///
	/// # Arguments
	/// * `key_code` - 検索するキーコード
	///
	/// # Returns
	/// * 検索結果
	fn find_menu_item(&self, e: Event) -> String {
		// 全メニューアイテムを走査
		for (id, menu_item) in self.menu_items.iter() {
			// メニューアイテムに関連付けられたキーコード
			let key_code = menu_item.1;

			// 修飾キーなしでマッチング
			if Event::Key(KeyEvent { code: key_code, modifiers: KeyModifiers::NONE }) == e {
				return id.clone();
			}
		}
		// 該当なし
		return "".to_string();
	}

	/// メニューを表示します。
	pub fn show(&mut self) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
		// use crossterm::style::Print;
		// use crossterm::terminal::{Clear, ClearType};
		// use std::io::Write;
		// let (_, y) = cursor::position()?;

		// TODO: メニュー項目の正しい判定

		// デフォルトポジション
		let mut current_section = "A".to_string();

		let mut prev_current_section = "".to_string();

		// キーイベントループ
		loop {
			// メニューを表示
			// execute!(stdout, cursor::MoveTo(0, y))?;

			// メニュー項目を表示します。
			reset_cursor_position(&prev_current_section)?;
			self.show_menuitems(&current_section)?;

			prev_current_section = current_section.clone();

			// 次のキー操作を待ちます。(*BLOCKING)
			let key = event::read()?;

			// 初めにメニュー項目を検索します。
			let id = self.find_menu_item(key);
			if id != "" {
				// キーがヒットした
				current_section = id;
				continue;
			}

			match key {
				// [Ctrl] + [C] to quit.
				Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => return Ok("".to_string()),
				// [Q] to quit.
				Event::Key(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE }) => return Ok("".to_string()),
				// [Up]
				Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE }) => {
					current_section = get_next_menuitem(&current_section, "Up");
				}
				// [Down]
				Event::Key(KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE }) => {
					current_section = get_next_menuitem(&current_section, "Down");
				}
				// [Enter]
				Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => break,
				// Else
				_ => {}
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
	// TODO: メニューアイテムの正しい判定
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

/// カーソル位置をY方向に移動します。
///
/// # Arguments
/// * `amount` - 移動量
#[allow(unused)]
fn move_y(amount: i16) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();

	// 現在の位置
	let (x, mut y) = cursor::position()?;
	if amount > 0 {
		y = y + amount as u16;
	} else {
		y = y.saturating_sub(amount.abs() as u16);
	}
	// 移動
	execute!(stdout, cursor::MoveTo(x, y))?;
	return Ok(());
}

/// カーソルの位置をリセットします。
///
/// # Arguments
/// * `prev_current_section` - 基準となる(=現在の)メニューアイテム
fn reset_cursor_position(prev_current_section: &str) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();

	// TODO: メニューアイテムの正しい判定
	if prev_current_section == "A" {
		let (_, y) = cursor::position()?;
		execute!(stdout, cursor::MoveTo(0, y))?;
	} else if prev_current_section == "B" {
		let (_, y) = cursor::position()?;
		execute!(stdout, cursor::MoveTo(0, y - 1))?;
	} else if prev_current_section == "X" {
		let (_, y) = cursor::position()?;
		execute!(stdout, cursor::MoveTo(0, y - 2))?;
	}
	return Ok(());
}

/// メニュー項目を表示してユーザーに選択させるサンプルです。
fn run() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	// RAW MODE
	terminal::enable_raw_mode()?;

	// CLS
	if false {
		cls()?;
	}

	println!("↓どうしますか？");

	// メニューを構築
	let mut menu = MyMenuController::new();
	menu.add_menuitem("A", "FUNCTION A", KeyCode::Char('a'))?;
	menu.add_menuitem("B", "FUNCTION B", KeyCode::Char('b'))?;
	menu.add_menuitem("X", "EXIT", KeyCode::Char('x'))?;

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
