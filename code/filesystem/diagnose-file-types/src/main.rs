//!
//! ファイルの拡張子を分析する
//!

#[allow(unused)]
type FileHandler = dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;

/// ファイル走査
///
/// # Arguments
/// * `e` パス
/// * `handler` ファイルハンドラー
fn search(
	path: &std::path::Path,
	handler: &mut dyn FnMut(&std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>,
) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	if !path.exists() {
		println!("[TRACE] invalid path {}", path.to_str().unwrap());
		return Ok(());
	}
	if path.is_dir() {
		let name = path.file_name().unwrap_or_default();
		if name == "node_modules" {
			return Ok(());
		}
		if name == "target" {
			return Ok(());
		}
		let it = std::fs::read_dir(path)?;
		for e in it {
			let entry = e.unwrap();
			let entry_path = entry.path();
			search(&entry_path, handler)?;
		}
		return Ok(());
	} else if path.is_file() {
		return handler(path);
	} else {
		println!("[WARN] 不明なファイルシステム {:?}", path);
	}
	return Ok(());
}

struct Calculator {
	map: std::collections::HashMap<String, u32>,
}

impl Calculator {
	pub fn new() -> Self {
		return Self {
			map: std::collections::HashMap::new(),
		};
	}

	pub fn summary(&mut self) {
		let mut total: u32 = 0;
		for e in &self.map {
			println!("{}\t{}", e.0, e.1);
			total += e.1;
		}
		println!("TOTAL: {:?}", total);
	}

	pub fn diagnose(&mut self, path: &std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// ファイル名
		let name = path.file_name().unwrap_or_default();
		let name = name.to_str();
		if name.is_none() {
			println!("[WARN] Cannot retrieve file name of [{:?}].", path);
			return Ok(());
		}
		let name = name.unwrap();

		// 拡張子
		let result = path.extension();
		if result.is_none() {
			// println!("[WARN] Cannot retrieve ext of [{:?}].", path);
			return Ok(());
		}
		let extension = result.unwrap();
		let result = extension.to_str();
		if result.is_none() {
			println!("[WARN] Cannot retrieve ext of [{:?}].", extension);
			return Ok(());
		}
		let extension = result.unwrap();

		println!("> [{:?}] >> [{}] + [{}]", path, name, extension);

		let value = self.map.get_key_value(extension);
		if value.is_none() {
			// 1
			println!("NEW FILE> [{}] = 1", &extension);
			self.map.insert(extension.to_string(), 1);
		} else {
			// 加算
			let current = value.unwrap().1;
			let new_value = *current;
			println!("INCREMENT> [{}] = ({})", &extension, new_value);
			self.map.insert(extension.to_string(), new_value + 1);
		}
		return Ok(());
	}
}

/// エントリーポイント
#[allow(unused)]
fn main() {
	// コマンドライン引数
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_millis(1000));
		return;
	}
	let path_to_target = &args[0];
	let path = std::path::Path::new(path_to_target);

	// 計算機
	let mut calculator = Calculator::new();

	// ハンドラー
	let mut handler = |arg: &std::path::Path| -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		calculator.diagnose(arg)?;
		return Ok(());
	};

	// ファイル走査
	let result = search(&path, &mut handler);
	if result.is_err() {
		println!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(3));
		return;
	}

	// サマリーを表示
	calculator.summary();
}
