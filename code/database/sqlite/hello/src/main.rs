extern crate sqlite;

fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let connection = sqlite::open(":memory:").unwrap();

	// 表を作成しています。
	{
		connection.execute(
			r#"
				CREATE TABLE USERS(
					ID VARCHAR(999) NOT NULL,
					NAME VARCHAR(255) NOT NULL,
					PRIMARY KEY(ID)
				)"#,
		)?;
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
			println!(
				"id={}, name={}",
				statement.read::<String>(0)?,
				statement.read::<String>(1)?
			);
		}
	}

	return Ok(());
}

fn main() {
	// メモリ上の仮想データベースを開きます。
	let result = run();
	if result.is_err() {
		println!("[ERROR] {:?}", result.err().unwrap());
		return;
	}

	println!("Ok.");
}
