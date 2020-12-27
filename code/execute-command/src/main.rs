#[allow(unused)]
fn execute_windows_command() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// let path = "yarn.cmd"; // will cause not found.
	let path = "C:\\Program Files (x86)\\Yarn\\bin\\yarn.cmd";
	let mut command = std::process::Command::new(path);
	let status = command.args(&["install"]).spawn()?.wait()?;
	println!("[{}]", status.code().unwrap());
	return Ok(());
}

#[allow(unused)]
fn execute_yarn_help_on_shell() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// コマンドを実行して標準出力を受け取る方法
	let mut command = std::process::Command::new("cmd.exe");
	let status = command.args(&["/C", "yarn.cmd", "--help"]).spawn()?.wait()?;
	println!("exit code: [{}]", status.code().unwrap());
	return Ok(());
}

#[allow(unused)]
fn execute_windows_command_shell() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// コマンドを実行して放置
	let mut command = std::process::Command::new("cmd.exe");
	let result = command.args(&["/C", "ECHO %PATH%"]).output().expect("failed to execute process");
	let hello = result.stdout;
	let content = std::str::from_utf8(&hello).unwrap();
	println!("[{}]", content);
	return Ok(());
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// execute_windows_command()?;
	// execute_windows_command_shell()?;
	execute_yarn_help_on_shell()?;
	return Ok(());
}
