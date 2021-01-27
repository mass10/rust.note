/// 一行の入力を得ます。
fn input_line() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	return line.trim().to_string();
}

/// プロンプトを出してユーザー確認を得ます。
#[allow(unused)]
fn confirm() -> bool {
	let answer = input_line().to_uppercase();
	return match answer.as_str() {
		"Y" => true,
		"YES" => true,
		_ => false,
	};
}

#[allow(unused)]
/// Execute command in the OS shell.
pub fn shell_exec(commands: &String) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// Try to execute command for Windows
	let mut command = std::process::Command::new("cmd.exe");
	command.arg("/C");
	command.arg(commands);
	let mut response = command.spawn()?;
	let status = response.wait()?;
	if !status.success() {
		let exit_code = status.code().unwrap();
		return Ok(exit_code);
	}
	return Ok(0);
}

/// Visual Studio 2015 の dumpbin.exe を実行します。
fn dumpbin2015(args: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	let dumpbin_path = "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\dumpbin.exe";
	let mut command = std::process::Command::new(dumpbin_path);
	let mut command = command.args(&[args]).spawn()?;
	let status = command.wait()?;
	if !status.success() {
		let exit_code = status.code().unwrap();
		// println!("[WARN] yarn exited with status: {}", exit_code);
		return Ok(exit_code);
	}
	return Ok(0);
}

fn usage() {
	println!("USAGE:");
	println!("    dumpbin.bat [path to .exe/.dll]");
}

/// エントリーポイントの定義です。
fn main() {
	// コマンドラインオプション(コマンド名を除く)
	let args: Vec<String> = std::env::args().skip(1).collect();

	if args.len() == 0 {
		usage();
		return;
	}

	let result = dumpbin2015(&args[0]);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}

	// std::thread::sleep(std::time::Duration::from_secs(3));

	println!();
	println!("Ok.");
	println!();
}
