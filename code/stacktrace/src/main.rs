// #![feature(backtrace)]

extern crate backtrace;

struct Logger {}

impl Logger {
	pub fn trace(text: &str) {
		println!("{}", text);
	}
}

struct Application {}

impl Application {
	pub fn new() -> Application {
		return Application {};
	}

	fn dump_backtrace(self: &Application) {
		// 呼び出しフレームが列挙されます
		backtrace::trace(|frame| {
			// ひとつずつ呼びだされる
			println!("---");
			// 関数のアドレス？
			let ip = frame.ip();
			println!("[TRACE] ip: {:?}", ip);
			//
			let symbol_address = frame.symbol_address();
			println!("[TARCE] symbol_address: {:?}", symbol_address);

			// Resolve this instruction pointer to a symbol name
			backtrace::resolve_frame(frame, |symbol| {
				if let Some(name) = symbol.name() {
					println!("[TRACE] name: {:?}", name);
				}
				if let Some(filename) = symbol.filename() {
					println!("[TRACE] filename: {:?}", filename);
				}
			});

			println!();
			true // keep going to the next frame
		});
	}

	pub fn run(self: &Application) {
		Logger::trace("### START ###");
		Logger::trace("コニチハ！");
		self.dump_backtrace();
		Logger::trace("--- END ---");
	}
}

fn main() {
	let app = Application::new();
	app.run();
}
