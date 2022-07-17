use crate::io;

use crate::configuration::Configuration;

/// 拡張子診断クラス
///
/// lifetime 'a の明示によって、`Calculator` と `Configuration` の生存期間をコンパイル時に保証
#[allow(unused)]
pub struct Application<'a> {
	/// コンフィギュレーションへの参照
	conf: &'a Configuration,
	/// 拡張子と件数を管理します。
	map: std::collections::BTreeMap<String, u32>,
}

impl<'a> Application<'a> {
	/// `Calculator` の新しいインスタンスを返します。
	///
	/// # Returns
	/// `Calculator` の新しいインスタンス
	pub fn new(conf: &'a Configuration) -> Self {
		return Self {
			conf: conf,
			map: std::collections::BTreeMap::new(),
		};
	}

	/// アプリケーションを実行します。
	pub fn run(&mut self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		// コンフィギュレーション
		let conf = self.conf;
		let path = std::path::Path::new(&conf.path_to_run);

		// ハンドラー
		let mut handler = |arg: &std::path::Path| -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
			self.diagnose(arg)?;
			return Ok(());
		};

		// ファイル走査
		io::search(&conf, &path, &mut handler)?;

		// サマリーを表示
		self.summary();

		return Ok(());
	}

	/// 診断結果を出力します。
	fn summary(&self) {
		let mut total: u32 = 0;
		for e in &self.map {
			println!("{}\t{}", e.0, e.1);
			total += e.1;
		}
		println!("TOTAL: {:?}", total);
	}

	/// ファイルパスを診断します。
	///
	/// # Arguments
	/// `path` ファイルのパス
	fn diagnose(&mut self, path: &std::path::Path) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
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

		println!("> [{}] >> [{}] + [{}]", path.as_os_str().to_str().unwrap(), name, extension);

		let value = self.map.get_key_value(extension);
		if value.is_none() {
			// 初めての拡張子
			self.map.insert(extension.to_string(), 1);
		} else {
			// 既知の拡張子
			let current = value.unwrap().1;
			let new_value = *current;
			self.map.insert(extension.to_string(), new_value + 1);
		}
		return Ok(());
	}
}
