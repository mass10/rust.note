//!
//! ロギング関数の実装を試みる
//!

/// 現在のタイムスタンプを返します。
///
/// ### Returns
/// タイムスタンプ
fn get_current_timestamp() -> String {
	let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
	return format!("{}", timestamp);
}

/// 愚直引数バージョン
///
/// # Arguments
/// * `args` - 引数の配列を指定します。
fn info1(args: &[&dyn std::fmt::Display]) {
	print!("{} [INFO1] ", get_current_timestamp());
	for arg in args {
		print!("{}", arg);
	}
	println!();
}

#[macro_export]
macro_rules! info2 {
    ($($arg:tt)*) => ({
        print!("{} [INFO2] ", get_current_timestamp());
        print!($($arg)*);
    })
}

// こういうことがやりたい
// fn info2(args: ...&dyn std::fmt::Display) {
// 	for arg in args {
// 		print!("{}", arg);
// 	}
// 	println!();
// }

fn main() {
	info1(&[&"### start ###", &"aaa", &900, &1.23]);
	info1(&[&"### start ###".to_string()]);
	info2!("{} {}", 1, "abc");
}
