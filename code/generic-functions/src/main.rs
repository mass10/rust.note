/// 複数の型に対して同じインターフェイスを付けたい場合
trait ObjectSomethingDescribable {
	fn describe(&self) -> String;
}

impl ObjectSomethingDescribable for &str {
	fn describe(&self) -> String {
		return format!("(&str) {}", &self);
	}
}

impl ObjectSomethingDescribable for String {
	fn describe(&self) -> String {
		return format!("(String) {}", &self);
	}
}

impl ObjectSomethingDescribable for i8 {
	fn describe(&self) -> String {
		return format!("(i8) {}", &self);
	}
}

impl ObjectSomethingDescribable for i16 {
	fn describe(&self) -> String {
		return format!("(i16) {}", &self);
	}
}

impl ObjectSomethingDescribable for i32 {
	fn describe(&self) -> String {
		return format!("(i32) {}", &self);
	}
}

/// ジェネリック関数
///
/// * いくつかの不確定な要素を同様に扱いたい場合
/// * 抽象的な基本型に対して同じ操作をしたい
/// * 同じ操作でそれぞれの実装を呼びだしたい
///
/// Java で interface を用いる場面に似ているかも
///
/// # Arguments
/// * `unknown` - 不明なオブジェクト
fn test_describe<T: ObjectSomethingDescribable>(unknown: T) {
	println!("フォーマットされた形式: {}", unknown.describe());
}

/// 不明なオブジェクト T を出力します。
///
/// # Arguments
/// * `unknown` - 不明なオブジェクト
fn trace_unknown_object<T: std::fmt::Debug>(unknown: &T) {
	println!("{:?}", unknown);
}

/// エントリーポイント
fn main() {
	// 型パラメーターを明示的に宣言できる。
	test_describe::<&str>("エドラダワー(&str)");
	test_describe("エドラダワー(&str)");
	test_describe(String::from("エドラダワー(String)"));

	test_describe::<i8>(0x7F);

	test_describe::<i16>(0x7FFF);

	// 型パラメーターを省略する i32 になる
	test_describe(1);

	// 型は省略できる。
	test_describe(256);

	// 型パラメーター自体も省略できる。
	test_describe(2147483647);

	trace_unknown_object(&["111", "bbb", "~~~"]);
}
