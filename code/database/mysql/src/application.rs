extern crate yaml_rust;
extern crate mysql;

use std::fs::File;
use std::io::Read;
use yaml_rust::yaml::YamlLoader;
	

fn configure() -> String {
	match File::open("conf/settings.yaml") {
		Err(error) => {
			println!("{:?}", error);
			return String::new();
		},
		Ok(mut file) => {
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();
			return content;
		},
	};
}

// fn configure2() -> Option<yaml_rust::Yaml> {

// 	let content = configure();
// 	if content == "" {
// 		println!("[WARN] exit.");
// 		return None;
// 	}
// 	let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
// 	let doc = &docs[0];
// 	return Some(doc);
// }

pub struct Application {

	_connection: Option<mysql::Conn>,
}

impl Application {

	// fn new<'a>() -> Application<'a> {
	//  let mut app = Application { _connection: None };
	//  return &app;
	// }

    pub fn new() -> Application {
        let app = Application { _connection: None };
        return app;
    }

	fn create_new_connection() -> Option<mysql::Conn> {

		let content = configure();
		if content == "" {
			panic!("[ERROR] no configuration settings.");
		}
		let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
		let doc = &docs[0];
		let host = doc["database"]["host"].as_str().unwrap();
		let port = doc["database"]["port"].as_str().unwrap();
		let user = doc["database"]["user"].as_str().unwrap();
		let password = doc["database"]["password"].as_str().unwrap();

		//========== debug ==========
		if false {
			println!("{:?}", doc["key1"].as_str().unwrap());
			println!("{:?}", doc["key2"].as_str().unwrap());
			println!("{:?}", doc);
			println!("{:?}", doc["database"]);
			println!("{:?}", doc["database"]["host"].as_str());
			println!("{:?}", doc["database"]["port"].as_str());
			println!("{:?}", doc["key3"].as_f64());
			println!("{:?}", doc["key4"].is_badvalue());
		}

		let connection_string = format!("mysql://{}:{}@{}:{}/accounts", user, password, host, port);

		// もしかしてコネクションが複製されるのでは...
		let pool = mysql::Pool::new(connection_string).unwrap();
		let conn = pool.get_conn().unwrap().unwrap();
		return Some(conn);
	}

	pub fn run(&mut self) {

		println!("[INFO] ### START ###");

		//========== test MySQL Connection ==========
		let pool = self.open_connection();

		//========== create a table ==========
		{
			println!("[INFO] creating a temporary table...");
			for row in pool.prep_exec("select connection_id()", ()).unwrap() {
				let connection_id : i64 = mysql::from_row(row.unwrap());
				println!("[TRACE] connection_id: {}", connection_id);
			}
			let sql = r"
			CREATE TEMPORARY TABLE ACCOUNT(
				MAIL NVARCHAR(999) NOT NULL,
				PRIMARY KEY(MAIL))
			";
			let _ = pool.prep_exec(sql, ()).unwrap();
			// ↓投げてない？？？
			// let mut stmt = pool.prepare(sql).unwrap();
			// stmt.execute(()).unwrap();
			println!("[INFO] Ok.");
			for row in pool.prep_exec("select connection_id()", ()).unwrap() {
				let connection_id : i64 = mysql::from_row(row.unwrap());
				println!("[TRACE] connection_id: {}", connection_id);
			}
		}

		{
			println!("[INFO] creating records...");
			for row in pool.prep_exec("select connection_id()", ()).unwrap() {
				let connection_id : i64 = mysql::from_row(row.unwrap());
				println!("[TRACE] connection_id: {}", connection_id);
			}
			let sql = r"
			INSERT INTO ACCOUNT VALUES(?)
			";
			let mut stmt = pool.prepare(sql).unwrap();
			stmt.execute((String::from("jimi.hendrix@i.softbank.jp"),)).unwrap();
			stmt.execute((String::from("janis.joplin@gmail.com"),)).unwrap();
			stmt.execute((String::from("paul.kossof@gmail.com"),)).unwrap();
			println!("[INFO] Ok.");
		}

		//========== test MySQL Connection ==========
		{
			println!("[INFO] enumerating records...");
			let sql = "SELECT MAIL FROM ACCOUNT";
			let mut stmt = pool.prepare(sql).unwrap();
			for row in stmt.execute(()).unwrap() {
				let name : String = mysql::from_row(row.unwrap());
				// let name = String::from_utf8(name).unwrap();
				println!("[TRACE] name: {}", name);
			}
			println!("[INFO] Ok.");
		}

		{
			for row in pool.prep_exec("select connection_id()", ()).unwrap() {
				let connection_id : i64 = mysql::from_row(row.unwrap());
				println!("[TRACE] connection_id: {}", connection_id);
			}
		}

		println!("[INFO] --- end ---");
	}

	fn open_connection(&mut self) -> &mut mysql::Conn {
		if self._connection.is_some() {
			return self._connection.as_mut().unwrap();
		}
		self._connection = Application::create_new_connection();
		let conn = self._connection.as_mut().unwrap();
		return conn;
	}
}

// fn open_connection() -> Option<&'static mysql::Pool> {

//  let content = configure();
//  if content == "" {
//      println!("[WARN] exit.");
//      return None;
//  }
//  let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
//  let doc = &docs[0];
//  let host = doc["database"]["host"].as_str().unwrap();
//  let port = doc["database"]["port"].as_str().unwrap();
//  let user = doc["database"]["user"].as_str().unwrap();
//  let password = doc["database"]["password"].as_str().unwrap();
//  let connection_string = format!("mysql://{}:{}@{}:{}/accounts", user, password, host, port);
//  println!("{:?}", connection_string);
//  let pool = mysql::Pool::new(connection_string).unwrap();
//  return Some(&pool);
// }

