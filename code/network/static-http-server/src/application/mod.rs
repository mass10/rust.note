//!
//! アプリケーションのコア実装です。
//!

use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
// use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// タイムスタンプ "%Y-%m-%d %H:%M:%S%.3f" を返します。
#[allow(unused)]
pub fn get_current_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

#[get("/*")]
async fn index(req: HttpRequest) -> impl Responder {
	let path = req.path();
	println!("{} [TRACE] GET {}", get_current_timestamp(), path);
	format!("GET {}\n", path)
}

// #[get("/")]
// async fn hello(path: web::Path<String>) -> impl Responder {
// 	println!("{} [TRACE] GET {}", get_current_timestamp(), path);
// 	format!("GET {}\n", path)
// }

async fn greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("");
	println!("{} [TRACE] GET {}", get_current_timestamp(), &name);
	format!("GET {}\n", &name)
}

pub struct Application {}

impl Application {
	/// `Application` の新しインスタンスを作成します。
	///
	/// ## Returns
	// `Application` の新しいインスタンス
	pub fn new() -> Application {
		return Application {};
	}

	/// ウェブサーバーを起動します。
	pub async fn run(
		&mut self,
		server: &str,
		port: i32,
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let server_string = format!("{}:{}", server, port);

		// ウェブサーバーを起動
		// let app = App::new().service(index).route("/", web::get().to(greet));
		// let app = App::new().route("/", web::get().to(greet));

		let _context = HttpServer::new(|| {
			App::new()
				.service(index)
				.route("/{name}", web::get().to(greet))
		})
		.bind(server_string)?
		.run()
		.await;

		println!("Ok.");

		return Ok(());
	}
}
