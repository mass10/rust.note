extern crate hyper;
extern crate futures;
extern crate chrono;

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};


struct HelloWorld;

const PHRASE: &'static str = "{\"status\": \"Hello, World!\"}\n";

impl Service for HelloWorld {

	type Request = Request;
	type Response = Response;
	type Error = hyper::Error;
	type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

	fn call(&self, _req: Request) -> Self::Future {
		println!("{} [TRACE] accept.", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
		Box::new(futures::future::ok(
			Response::new()
				.with_header(ContentLength(PHRASE.len() as u64))
				.with_body(PHRASE)
		))
	}
}

fn main() {

	// println!("{} [TRACE] accept.", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
	let addr = "127.0.0.1:3000".parse().unwrap();
	let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
	server.run().unwrap();
}
