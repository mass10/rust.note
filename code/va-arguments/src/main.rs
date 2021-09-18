/// 愚直引数バージョン
///
/// # Arguments
/// * `args` - 引数の配列を指定します。
fn info(args: &[&dyn std::fmt::Display]) {
	for arg in args {
		print!("{}", arg);
	}
	println!();
}

fn main() {
	info(&[&"### start ###", &"aaa", &900, &1.23]);
	info(&[&"### start ###".to_string()]);
}
