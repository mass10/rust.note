mod csv_loader;
mod database_connection;
mod user_manager;

struct Application;

impl Application {
	pub fn run(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// CSV ファイルを読み込み
		let mut loader = csv_loader::CsvLoader::new();
		loader.open_csv_file(".rustfmt.toml")?;

		// インポート
		loop {
			let name = "";
			let email = "";
			let user_manager = user_manager::UserManager::new();
			user_manager.register_new_user(name, email)?;
			break;
		}

		loader.close();

		return Ok(());
	}
}

/// エントリーポイント
fn main() {
	let mut app = Application {};
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] ERROR! REASON: {:?}", result.err());
		return;
	}
}
