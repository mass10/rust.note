pub fn red<T: std::fmt::Display>(s: T) -> String {
	return format!("\x1b[31m{}\x1b[39m", s);
}

pub fn green<T: std::fmt::Display>(s: T) -> String {
	return format!("\x1b[32m{}\x1b[39m", s);
}

pub fn result(b: bool) -> String {
	match b {
		true => green(b),
		false => red(b)
	}
}
