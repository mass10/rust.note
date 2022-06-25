/// 文字修飾の提供
mod decorator {

	pub fn black<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[30m{}\x1b[0m", s);
	}

	pub fn red<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[31m{}\x1b[0m", s);
	}

	pub fn green<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[32m{}\x1b[0m", s);
	}

	pub fn yellow<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[33m{}\x1b[0m", s);
	}

	pub fn blue<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[34m{}\x1b[0m", s);
	}

	pub fn magenta<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[35m{}\x1b[0m", s);
	}

	pub fn cyan<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[36m{}\x1b[0m", s);
	}

	pub fn white<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[37m{}\x1b[0m", s);
	}

	pub fn bold<T: std::fmt::Display>(s: T) -> String {
		return format!("\x1b[1m{}\x1b[0m", s);
	}

	pub const RED: &str = "\x1b[31m";

	pub const YELLOW: &str = "\x1b[33m";

	pub const GREEN: &str = "\x1b[32m";

	pub const STRONG: &str = "\x1b[1m";

	pub const CANCEL: &str = "\x1b[0m";

	pub struct ColorPrinter;

	impl ColorPrinter {
		pub fn new() -> Self {
			Self
		}

		pub fn print<T: std::fmt::Display>(&self, text: T) -> &Self {
			print!("{}", text);
			return self;
		}

		pub fn yellow<T: std::fmt::Display>(&self, text: T) -> &Self {
			print!("{YELLOW}{}", text);
			return self;
		}

		pub fn red<T: std::fmt::Display>(&self, text: T) -> &Self {
			print!("{RED}{}", text);
			return self;
		}

		pub fn off(&self) -> &Self {
			print!("{CANCEL}");
			return self;
		}

		pub fn eol(&self) -> &Self {
			println!("{CANCEL}");
			return self;
		}
	}
}

/// エントリーポイント
fn main() {
	use decorator::{black, blue, bold, cyan, green, magenta, red, white, yellow, CANCEL, GREEN, STRONG};

	println!("{}", black("BLACK EXAMPLE"));
	println!("{}", red("RED EXAMPLE"));
	println!("{}", green("GREEN EXAMPLE"));
	println!("{}", yellow("YELLOW EXAMPLE"));
	println!("{}", blue("BLUE EXAMPLE"));
	println!("{}", magenta("MAGENTA EXAMPLE"));
	println!("{}", cyan("CYAN EXAMPLE"));
	println!("{}", white("WHITE EXAMPLE"));

	println!("{}", bold(black("BLACK EXAMPLE (STRONG)")));
	println!("{}", bold(red("RED EXAMPLE (STRONG)")));
	println!("{}", bold(green("GREEN EXAMPLE (STRONG)")));
	println!("{}", bold(yellow("YELLOW EXAMPLE (STRONG)")));
	println!("{}", bold(blue("BLUE EXAMPLE (STRONG)")));
	println!("{}", bold(magenta("MAGENTA EXAMPLE (STRONG)")));
	println!("{}", bold(cyan("CYAN EXAMPLE (STRONG)")));
	println!("{}", bold(white("WHITE EXAMPLE (STRONG)")));

	println!("{GREEN}{STRONG}STRONG GREEN{CANCEL}{CANCEL}");

	// へんなプリンター実装
	let printer = decorator::ColorPrinter::new();
	printer.yellow("黄色？").red("赤？").off().print("標準に戻ってる？").eol();
}
