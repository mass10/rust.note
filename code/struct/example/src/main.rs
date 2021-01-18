/**
 * 座標を表す構造体を定義します。
 */
#[allow(unused)]
#[derive(Debug)]
struct Point {
	/// X
	x: i32,
	/// Y
	y: i32
}

/**
 * 文字列の属性を一つだけ持つ構造体を定義します。
 *
 * メンバーは名前を持ちません。
 */
struct Name(String);

/**
 * 文字列の属性を二つ持つ構造体を定義します。
 *
 * メンバーは名前を持ちません。
 */
struct Tuple2(String, String);

/// エントリーポイント
#[allow(unused)]
fn main() {

	// immutable(読み取り専用)の座標を扱います。
	{
		let p = Point{x: 0, y: 0};
		println!("{:?}", p);
	}

	// 変更可能の座標を扱います。
	{
		let mut p = Point{x: 0, y: 0};
		p.x = 100;
		p.y = 100;
		println!("{:?}", p);
	}
	
	// 文字列の属性を一つ持つ単純な構造体を扱います。
	{
		let name = Name("No Name".to_string());
		println!("{}", name.0);
	}

	// 文字列の属性を二つ持つ単純な構造体を扱います。
	{
		let t = Tuple2("left".to_string(), "right".to_string());
		println!("left: [{}], right: [{}]", t.0, t.1);
	}
}
