///
/// 構造体 A
///
struct A {
	/// 名前
	name: String,
}

/// オブジェクト `A` の参照を受け取ります。(borrowed)
/// * `a` ... type `A`
fn dump(a: &A) {
	println!("{}", a.name);

	// CANNOT MODIFY FIELD VALUE OF BORROWED OBJECT.
	// a.name = "".to_string();
}

/// エントリーポイント
fn main() {
	// 構造体 `A` の immutable な実体
	let a = A { name: "Hahaha".to_string() };

	// オブジェクトをダンプ
	dump(&a);

	println!("Ok.");
}
