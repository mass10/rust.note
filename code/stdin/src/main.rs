use std::io::{Write};

fn read(reader: std::io::Stdin) -> std::result::Result<(), Box<dyn std::error::Error>> {
	loop {
		print!("? ");
		std::io::stdout().flush().unwrap();
		// line には 0d,0a も入ってきます。
		let mut line = String::new();
		let read = reader.read_line(&mut line)?;
		if read == 0 {
			// シグナル発生
			break;
		}
		let line = line.trim_end();
		println!("> [{}] {}", line, read);
	}
	return Ok(());
}

fn main() {
	let result = read(std::io::stdin());
	if result.is_err() {
		println!("{}", result.err().unwrap());
	}
}
