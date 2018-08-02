fn read(stdin: std::io::Stdin) {

	let reader = stdin;

	loop {
		let mut line = String::new();
		let ret = reader.read_line(&mut line);
		if ret.is_err() {
			break;
		}
		if ret.unwrap() == 0 {
			break;
		}
		print!("{}", line);
	}
}

fn main() {

	read(std::io::stdin());
}
