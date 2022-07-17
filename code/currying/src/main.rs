/// 特別なプリンターを返します。
/// 
/// # Arguments
/// * `prefix` プレフィクス
fn printer(prefix: &str) -> impl Fn (i32) + '_ {
	return move |n: i32| {
		println!("{}{}", prefix, n);
	};
}

/// `base` をもとに加算する関数を返します。
/// 
/// # Arguments
/// * `base` もとの数
fn multiply(base: i32) -> impl Fn(i32) -> i32 {
	let f = move |n| {
		return base * n;
	};
	return f;
}

/// アプリケーションのエントリーポイントです。
fn main() {
	let printer = printer("> ");

	printer(multiply(1)(2)); // > 2
	printer(multiply(2)(3)); // > 6
	printer(multiply(3)(4)); // > 12
}
