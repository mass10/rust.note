extern crate hyper;
extern crate futures;
extern crate chrono;

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

struct Logger;

impl Logger {

	fn put(text: String) {
		println!("{} [TRACE] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), text);
	}

	fn put_str(text: &str) {
		Logger::put(text.to_string());
	}
}

struct HelloWorld;


impl Service for HelloWorld {

	type Request = Request;
	type Response = Response;
	type Error = hyper::Error;
	type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

	fn call(&self, _req: Request) -> Self::Future {

		const PHRASE: &'static str = "{\"status\": \"Hello, World!\"}\n";

		Logger::put_str("accept.");
		Box::new(futures::future::ok(
			Response::new()
				.with_header(ContentLength(PHRASE.len() as u64))
				.with_body(PHRASE)
		))
	}
}

fn main() {

	Logger::put_str("### start ###");
	let addr = "127.0.0.1:3000".parse().unwrap();
	let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
	server.run().unwrap();
}
