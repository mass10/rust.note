//!
//! オブジェクト指向プログラミング
//!
//! * 綺麗でスリムなオブジェクト指向プログラミングの試み
//!     * インターフェイス(=trait)、およびコンストラクト関数のみを公開
//!     * 実体(=struct)を隠蔽する
//!

mod application;
mod configuration;

/// エントリーポイント
fn main() {
	// ========== CONFIGURATION ==========

	// コンフィギュレーション
	let result = configuration::configure();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("ERROR: {:?}", error);
		return;
	}
	let conf = result.unwrap();

	// ========== LAUNCH APPLICATION ==========

	// アプリケーションを初期化して
	let app = application::create_new(conf);

	// 実行
	let result = app.run();
	if result.is_err() {
		println!("ERROR: {}", result.unwrap_err());
		return;
	}
}
