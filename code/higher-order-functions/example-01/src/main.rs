/// アプリケーション固有のエラー
#[derive(Debug, Clone)]
pub struct ApplicationError {
	/// メッセージ
	pub message: String,
}

impl ApplicationError {
	/// エラーを生成する
	#[allow(unused)]
	pub fn new(message: &str) -> ApplicationError {
		return ApplicationError { message: message.to_string() };
	}
}

impl std::fmt::Display for ApplicationError {
	/// [std::fmt::Display] としての振る舞いを実装しています。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for ApplicationError {
	/// [std::error::Error] としての振る舞いを実装しています。
	fn description(&self) -> &str {
		&self.message
	}
}


type AbstractTask = Box<dyn Fn(&str) -> std::result::Result<(), Box<dyn std::error::Error>>>;

/// とある操作01
fn function01(message: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	println!("[TRACE] <function01()> {}", message);
	return Ok(());
}

/// とある操作02
fn function02(message: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	println!("[TRACE] <function02()> {}", message);
	return Ok(());
}

/// とある操作(nop)
fn nop(_: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	return Ok(());
}

/// 要求された識別子に該当するファンクションを返す関数
fn get_function(function_id: &str) -> Result<AbstractTask, Box<dyn std::error::Error>> {
	let operation = match function_id {
		"01" => function01,
		"02" => function02,
		_ => nop,
	};
	return Ok(Box::new(operation));
}

/// 要求された識別子に該当するファンクションを実行する関数
fn execute_task(id: &str) -> Result<(), Box<dyn std::error::Error>> {
	let task = get_function(id)?;
	return task("こにちは");
}

/// エントリーポイント
fn main() {
	// コマンドライン引数
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		return;
	}

	// 第一引数がリクエストです。
	let function_id = &args[1];

	// 要求された機能に該当するファンクションを取得します。
	let result = execute_task(&function_id);
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] <main()> {}", error);
		return;
	}

	println!("[TRACE] Ok.");
}
