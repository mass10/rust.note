extern crate sqlite;

struct Application {

}

impl Application {

	pub fn new() -> Application {
	
		let app = Application{};
		return app;
	}

	pub fn open() -> sqlite::Connection {

		let connection = sqlite::open(":memory:").unwrap();
		return connection;
	}

	pub fn run(&mut self) -> std::result::Result<String, sqlite::Error> {

		let connection = Application::open();

		// 表を作成しています。
		{
			connection.execute("CREATE TABLE USERS(ID VARCHAR(999) NOT NULL, NAME VARCHAR(255) NOT NULL)")?;
		}

		// 表にレコードを作成しています。
		{
			let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
			statement.bind(1, "1")?;
			statement.bind(2, "John Lennon")?;
			statement.next()?;

			let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
			statement.bind(1, "2")?;
			statement.bind(2, "Paul McCartney")?;
			statement.next()?;

			let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
			statement.bind(1, "3")?;
			statement.bind(2, "Ringo Starr")?;
			statement.next()?;

			let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
			statement.bind(1, "4")?;
			statement.bind(2, "George Harrison")?;
			statement.next()?;
		}

		// 表のレコードを抽出しています。
		{
			let mut statement = connection.prepare("SELECT * FROM USERS")?;
			while let sqlite::State::Row = statement.next()? {
				println!("id={}, name={}",
					statement.read::<String>(0)?, statement.read::<String>(1)?);
			}
		}

		return Ok(String::from("DONE"));
	}
}

fn main() {

	// メモリ上の仮想データベースを開きます。
	let mut app = Application::new();
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] {:?}", result.err().unwrap());
		return;
	}
	println!("[INFO] {}", result.unwrap());
}
