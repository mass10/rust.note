#[macro_use]
extern crate crossterm;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

fn main() {
	let mut stdout = stdout();
	enable_raw_mode().unwrap();
	// 画面を消去します。
	execute!(stdout, Clear(ClearType::All)).unwrap();
	// 左上 [0, 0] へ移動します。
	execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

	loop {
		// 次のキー操作を待ちます。
		let key_press = read().unwrap();
		match key_press {
			Event::Key(KeyEvent {
				code: KeyCode::Char('l'),
				modifiers: KeyModifiers::CONTROL,
			}) => execute!(stdout, Clear(ClearType::All), Print("")).unwrap(),
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::ALT,
			}) => break,
			// [Ctrl][C] to quit.
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => break,
			// [Q] to quit.
			Event::Key(KeyEvent {
				code: KeyCode::Char('q'),
				modifiers: KeyModifiers::NONE,
			}) => break,
			// Else
			e => {
				// 画面を消去します。
				execute!(stdout, Clear(ClearType::All)).unwrap();
				// 左上 [0, 0] へ移動します。
				execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
				print!("> ");
				print!("KEY: {:?}", e);
				std::io::stdout().flush().unwrap();
			}
		}
	}
}
