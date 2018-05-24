fn function01(message: String) -> std::result::Result<(), String> {
	println!("[TRACE] <function01()> {}", message);
	return Ok(());
}

fn function02(message: String) -> std::result::Result<(), String> {
	println!("[TRACE] <function02()> {}", message);
	return Ok(());
}

fn nop(_: String) -> std::result::Result<(), String> {
	return Err(String::from("NOP"));
}

// ファンクションを返す関数
fn get_function(function_id: String) -> std::boxed::Box<Fn(String) -> std::result::Result<(), String>> {
	match function_id.as_str() {
		"01" => std::boxed::Box::new(function01),
		"02" => std::boxed::Box::new(function02),
		_ => std::boxed::Box::new(nop),
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		return;
	}
	let function_id = &args[1];
	// find operation
	let unknown = get_function(function_id.to_string());
	// invoke
	let result = unknown(String::from("こにちは"));
	println!("[TRACE] <run()> {:?}", result);
}
