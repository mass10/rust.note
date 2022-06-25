mod decorator {

	pub fn black(s: &str) -> String {
		return format!("\x1b[30m{}\x1b[0m", s);
	}

	pub fn red(s: &str) -> String {
		return format!("\x1b[31m{}\x1b[0m", s);
	}

	pub fn green(s: &str) -> String {
		return format!("\x1b[32m{}\x1b[0m", s);
	}

	pub fn yellow(s: &str) -> String {
		return format!("\x1b[33m{}\x1b[0m", s);
	}

	pub fn blue(s: &str) -> String {
		return format!("\x1b[34m{}\x1b[0m", s);
	}

	pub fn magenta(s: &str) -> String {
		return format!("\x1b[35m{}\x1b[0m", s);
	}

	pub fn cyan(s: &str) -> String {
		return format!("\x1b[36m{}\x1b[0m", s);
	}

	pub fn white(s: &str) -> String {
		return format!("\x1b[37m{}\x1b[0m", s);
	}

	pub fn bold(s: &str) -> String {
		return format!("\x1b[1m{}\x1b[0m", s);
	}
}

fn main() {
	println!("black         : {}", decorator::black("COLORING EXAMPLE"));
	println!("red           : {}", decorator::red("COLORING EXAMPLE"));
	println!("green         : {}", decorator::green("COLORING EXAMPLE"));
	println!("yellow        : {}", decorator::yellow("COLORING EXAMPLE"));
	println!("blue          : {}", decorator::blue("COLORING EXAMPLE"));
	println!("magenta       : {}", decorator::magenta("COLORING EXAMPLE"));
	println!("cyan          : {}", decorator::cyan("COLORING EXAMPLE"));
	println!("white         : {}", decorator::white("COLORING EXAMPLE"));

	println!("black   (bold): {}", decorator::bold(&decorator::black("COLORING EXAMPLE")));
	println!("red     (bold): {}", decorator::bold(&decorator::red("COLORING EXAMPLE")));
	println!("green   (bold): {}", decorator::bold(&decorator::green("COLORING EXAMPLE")));
	println!("yellow  (bold): {}", decorator::bold(&decorator::yellow("COLORING EXAMPLE")));
	println!("blue    (bold): {}", decorator::bold(&decorator::blue("COLORING EXAMPLE")));
	println!("magenta (bold): {}", decorator::bold(&decorator::magenta("COLORING EXAMPLE")));
	println!("cyan    (bold): {}", decorator::bold(&decorator::cyan("COLORING EXAMPLE")));
	println!("white   (bold): {}", decorator::bold(&decorator::white("COLORING EXAMPLE")));
}
