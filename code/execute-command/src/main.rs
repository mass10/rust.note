fn main() {
	let mut command = std::process::Command::new("cmd.exe");
	let result = command.args(&["/C", "ECHO HELLO"]).output().expect("failed to execute process");
	let hello = result.stdout;
	let content = std::str::from_utf8(&hello).unwrap();
	println!("[{}]", content);
}
