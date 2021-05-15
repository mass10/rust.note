//!
//! データベース関連の操作を提供します。
//!

extern crate sqlite;
extern crate uuid;

use uuid::Uuid;

fn generate_uuid4() -> String {
	let uuid = Uuid::new_v4();
	return format!("{}", uuid);
}

///
/// データベース接続
///
pub struct DatabaseConnection {
	// データベース接続
	_connection: Option<sqlite::Connection>,
}

impl DatabaseConnection {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// `DatabaseConnection` の新しいインスタンス
	pub fn new() -> DatabaseConnection {
		let instance = DatabaseConnection { _connection: None };
		return instance;
	}

	/// ユーザーの登録
	///
	/// # Arguments
	/// `name` ユーザーの名前
	/// `email` ユーザーのメールアドレス
	///
	/// # Returns
	/// 成否
	pub fn insert(&self, name: &str, email: &str) -> bool {
		println!("[TRACE] REGISTER... name: {}, email: {}", name, email);
		return true;
	}

	fn open(&mut self) -> &mut sqlite::Connection {
		// 既に開いている場合は既存の接続を返します。
		if self._connection.is_some() {
			return self._connection.as_mut().unwrap();
		}

		// メモリ上の仮想データベースを開きます。
		let connection = sqlite::open(":memory:").unwrap();
		self._connection = Some(connection);
		let connection = self._connection.as_mut().unwrap();
		return connection;
	}

	fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let connection = self.open();

		connection.execute("CREATE TABLE USERS(ID VARCHAR(999) NOT NULL, NAME VARCHAR(255) NOT NULL)")?;

		let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
		statement.bind(1, generate_uuid4().as_str())?;
		statement.bind(2, "John Lennon")?;
		statement.next()?;

		let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
		statement.bind(1, generate_uuid4().as_str())?;
		statement.bind(2, "Paul McCartney")?;
		statement.next()?;

		let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
		statement.bind(1, generate_uuid4().as_str())?;
		statement.bind(2, "Ringo Starr")?;
		statement.next()?;

		let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
		statement.bind(1, generate_uuid4().as_str())?;
		statement.bind(2, "George Harrison")?;
		statement.next()?;

		return Ok(());
	}
}
