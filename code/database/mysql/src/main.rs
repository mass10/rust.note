extern crate yaml_rust;
extern crate mysql;

use std::fs::File;
use std::io::Read;
use yaml_rust::yaml::YamlLoader;
	

// fn open() -> File {
// 	match File::open("conf/settings.yml") {
// 	    Some(file) => file,
// 	    None => None,
// 	}
// }

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

// fn configure2() -> Option<&yaml_rust::Yaml> {

// 	let content = configure();
// 	if content == "" {
// 		println!("[WARN] exit.");
// 		return None;
// 	}
// 	let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
// 	return Some(&docs[0]);
// }

struct Application {

	_connection: Option<mysql::Pool>,
}

impl Application {

	// fn new<'a>() -> Application<'a> {
	// 	let mut app = Application { _connection: None };
	// 	return &app;
	// }

	fn run(&mut self) {

		//========== configuration ==========
		let content = configure();
		if content == "" {
			println!("[WARN] exit.");
			return;
		}
		let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
		let doc = &docs[0];

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
			return;
		}

		let host = doc["database"]["host"].as_str().unwrap();
		let port = doc["database"]["port"].as_str().unwrap();
		let user = doc["database"]["user"].as_str().unwrap();
		let password = doc["database"]["password"].as_str().unwrap();
		let connection_string = format!("mysql://{}:{}@{}:{}/accounts", user, password, host, port);

		//========== test MySQL Connection ==========
		let pool = mysql::Pool::new(connection_string).unwrap();
		let sql = "SELECT MAIL FROM ACCOUNT";
		let mut stmt = pool.prepare(sql).unwrap();
	    for row in stmt.execute(()).unwrap() {
	        let name : String = mysql::from_row(row.unwrap());
	        // let name = String::from_utf8(name).unwrap();
	        println!("name: {}", name);
	    }
	}
}

// fn open_connection() -> Option<&'static mysql::Pool> {

// 	let content = configure();
// 	if content == "" {
// 		println!("[WARN] exit.");
// 		return None;
// 	}
// 	let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
// 	let doc = &docs[0];
// 	let host = doc["database"]["host"].as_str().unwrap();
// 	let port = doc["database"]["port"].as_str().unwrap();
// 	let user = doc["database"]["user"].as_str().unwrap();
// 	let password = doc["database"]["password"].as_str().unwrap();
// 	let connection_string = format!("mysql://{}:{}@{}:{}/accounts", user, password, host, port);
// 	println!("{:?}", connection_string);
// 	let pool = mysql::Pool::new(connection_string).unwrap();
// 	return Some(&pool);
// }

fn main() {

	// let mut app = Application::new();
	let mut app = Application { _connection: None };
	app.run();
}
