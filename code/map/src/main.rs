use std::collections::HashMap;

fn main() {

	let mut map = HashMap::new();

	// キーと値のセットを追加する
	map.insert("key-0", "value-0");
	map.insert("key-1", "value-1");
	map.insert("key-2", "value-2");

	// エントリーを削除する
	map.remove("UNDEFINED KEY"); // no panic
	map.remove("key-1");

	// キーと値のセットを追加する
	map.insert("日本語 の キー", "東京都 千代田区 丸の内");
	map.insert("日本語 の キー", "東京都 千代田区 丸の内 (上書き)"); // no panic

	// ERROR: cannot borrow as mutable
	// map["new key"] = "";

	// オブジェクトの文字列表現
	println!("{:?}", map);

	// キーに該当する値を表示
	println!("{:?}", map["key-0"]);

	// panic!
	// println!("{:?}", map["key-1"]);

	// None を表示(panic しない)
	println!("{:?}", map.get("undefined-key"));
}
