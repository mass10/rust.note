#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn main() {
	let mut server = Nickel::new();

	server.utilize(router! {
		// 有効なルーティング。完全一致のようだ。
		get "/json1" => |_req, _res| {
			format!("あいうえお")
		}
		// 全ての GET をひっかけるようだ。
		get "**" => |_req, _res| {
			format!("{:?}: [/] Hello world!", std::thread::current().id())
		}
		// 有効なルーティング。完全一致のようだ。
		post "/test1" => |_req, _res| {
			format!("{:?}: [/test1] -> accepted post data.", std::thread::current().id())
		}
		// 無効。これは何もひっかけられなかった。
		post "test2" => |_req, _res| {
			format!("{:?}: [test2]: accepted post data.", std::thread::current().id())
		}


	});

	let _result = server.listen("127.0.0.1:6767");
}
