extern crate sqlite;


struct Service {

	// データベース接続
	_connection: Option<sqlite::Connection>,
}

impl Service {

	fn init(&mut self) -> Result<(), sqlite::Error> {

		let connection = self.open();

		connection.execute("CREATE TABLE USERS(ID VARCHAR(999) NOT NULL, NAME VARCHAR(255) NOT NULL)")?;

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

		return Ok(());
	}

	fn open(&mut self) -> &mut sqlite::Connection {	

		// 既に開いている場合は既存の接続を返します。
		if self._connection.is_some() {
			return self._connection.as_mut().unwrap();
		}

		// メモリ上の仮想データベースを開きます。
		self._connection = Some(sqlite::open(":memory:").unwrap());
		let connection = self._connection.as_mut().unwrap();
		return connection;
	}

	fn dump(&mut self) {

		let connection = self.open();

		// 表のレコードを抽出しています。
		let mut statement = connection.prepare("SELECT * FROM USERS").unwrap();
		while let sqlite::State::Row = statement.next().unwrap() {
			println!("id={}, name={}", statement.read::<String>(0).unwrap(), statement.read::<String>(1).unwrap());
		}
	}
}

fn main() {

	let mut s = Service{
		_connection: None
	};

	let _ = s.init();

	s.dump();
}
