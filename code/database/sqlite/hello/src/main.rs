extern crate sqlite;

fn main() {

	// メモリ上の仮想データベースを開きます。
	let connection = sqlite::open(":memory:").unwrap();

	// 表を作成しています。
	{
		connection.execute("CREATE TABLE USERS(ID VARCHAR(999) NOT NULL, NAME VARCHAR(255) NOT NULL)").unwrap();
	}

	// 表にレコードを作成しています。
	{
		let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)").unwrap();
		statement.bind(1, "1").unwrap();
		statement.bind(2, "aaa").unwrap();
		statement.next().unwrap();
	}

	// 表のレコードを抽出しています。
	{
		let mut statement = connection.prepare("select * from users").unwrap();
		while let sqlite::State::Row = statement.next().unwrap() {
			println!("id={}, name={}",
				statement.read::<String>(0).unwrap(), statement.read::<String>(1).unwrap());
		}
	}
}
