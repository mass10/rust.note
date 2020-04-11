use file_diff::diff;
///
///
/// 日本語パス名への対応が未確認です。
///

/// 標準入力から一行の入力を得ます。
fn input() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	if ret.unwrap() == 0 {
		return String::new();
	}
	return (*line.trim()).to_string();
}

/// プロンプトを表示し、YES/NO の応答を読み取ります。
fn confirm() -> bool {
	let line = input();
	if line.to_uppercase() == "Y" {
		return true;
	}
	if line.to_uppercase() == "YES" {
		return true;
	}
	return false;
}

/// 二つのファイルが同一かどうかを調べます。
fn seems_to_be_same(source_path: &std::path::Path, destination_path: &std::path::Path) -> bool {
	// 元
	let left = std::fs::metadata(source_path).unwrap();
	// 先
	let right_attribute = std::fs::metadata(destination_path);
	if right_attribute.is_err() {
		// 先のファイルがみつからないようです。
		return false;
	}
	let right = right_attribute.unwrap();
	// サイズとタイムスタンプが同じなら同じとみなします。
	if left.len() == right.len() {
		if left.modified().unwrap() == right.modified().unwrap() {
			return true;
		}
	}
	// 中身を比較
	return diff(
		source_path.to_str().unwrap(),
		destination_path.to_str().unwrap(),
	);
}

/// ファイルごとに呼びだされるハンドラーです。
fn copy_file(source_path: &str, destination_path: &str) -> std::io::Result<()> {
	if seems_to_be_same(
		std::path::Path::new(source_path),
		std::path::Path::new(destination_path),
	) {
		return Ok(());
	}
	println!("ファイル {} を上書きしますか？(y/N)", destination_path);
	if !confirm() {
		return Ok(());
	}
	let _result = std::fs::copy(source_path, destination_path);
	return Ok(());
}

/// ディレクトリをコピーします。
fn xcopy(
	source_path: &str,
	destination_path: &str,
	handler: &dyn Fn(&str, &str) -> std::io::Result<()>,
) -> std::io::Result<()> {
	let source_path = std::path::Path::new(source_path);
	let destination_path = std::path::Path::new(destination_path);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(());
	}
	if source_path.is_dir() {
		let dir_name = source_path.file_name().unwrap();
		if dir_name == "node_modules" {
			return Ok(());
		}
		if dir_name == ".git" {
			return Ok(());
		}
		if dir_name == ".project" {
			return Ok(());
		}
		if dir_name == "dist" {
			return Ok(());
		}
		if dir_name == ".nuxt" {
			return Ok(());
		}
		if dir_name == ".vscode" {
			return Ok(());
		}
		if dir_name == ".circleci" {
			return Ok(());
		}
		if dir_name == ".github" {
			return Ok(());
		}
		if dir_name == "env" {
			return Ok(());
		}
		// コピー先にディレクトリを作成します。
		let result = std::fs::create_dir_all(destination_path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] reason: {}", error);
			return Ok(());
		}
		// ディレクトリ内エントリーを走査
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e.unwrap();
			let name = entry.file_name();
			let path = entry.path();
			let _ = xcopy(
				&path.to_str().unwrap(),
				destination_path.join(name).as_path().to_str().unwrap(),
				handler,
			);
		}
		return Ok(());
	}
	if source_path.is_file() {
		let file_name = source_path.file_name().unwrap().to_str().unwrap();
		if file_name == ".gitignore" {
			return Ok(());
		}
		if file_name == ".project" {
			return Ok(());
		}
		if file_name == "makefile" {
			return Ok(());
		}
		if file_name == "yarn-error.log" {
			return Ok(());
		}
		if file_name.starts_with(".env.") {
			return Ok(());
		}
		return handler(
			source_path.to_str().unwrap(),
			destination_path.to_str().unwrap(),
		);
	}
	return Ok(());
}

/// エントリーポイントです。
fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 3 {
		println!("path?");
		return;
	}
	let _ = xcopy(&args[1], &args[2], &copy_file);
}
