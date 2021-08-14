// use actix_web::{get, web, App, HttpServer, Responder};

mod application;

trait DigitParser<T> {
	fn parse_i32(&self) -> i32;
}

impl DigitParser<i32> for String {
	/// 文字列を数値に変換します。
	fn parse_i32(&self) -> i32 {
		return match self.trim().parse::<i32>() {
			Ok(n) => n,
			Err(_) => 0,
		};
	}
}

fn usage() {
	println!("Usage: bootstrap --server 127.0.0.1 --port 8080");
	println!();
}

#[actix_web::main]
async fn main() {
	// Server addr
	let mut server = String::new();
	// Server port
	let mut port = String::new();

	// Commandline options
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		if e.starts_with("--port=") {
			let value_part = &e[7..];
			println!("[TRACE] port: [{}]", value_part);
			port = value_part.to_string();
		} else if e.starts_with("--server=") {
			let value_part = &e[9..];
			println!("[TRACE] server: [{}]", value_part);
			server = value_part.to_string();
		}
	}

	if server == "" {
		server = "127.0.0.1".to_string();
		// return;
	}
	if port == "" {
		port = "8080".to_string();
		// return;
	}

	// Create a new web app
	let mut app = application::Application::new();
	let result = app.run(&server, port.parse_i32()).await;
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	// HttpServer::new(|| App::new().service(index))
	// 	.bind("127.0.0.1:8080")?
	// 	.run()
	// 	.await
}
