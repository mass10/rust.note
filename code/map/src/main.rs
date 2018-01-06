use std::collections::HashMap;

fn main() {

	let mut map = HashMap::new();

	map.insert("key-0", "value-0");
	map.insert("key-1", "value-1");
	map.insert("key-2", "value-2");

	map.remove("UNDEFINED KEY"); // no panic
	map.remove("key-1");

	map.insert("日本語 の キー", "東京都 千代田区 丸の内");
	map.insert("日本語 の キー", "東京都 千代田区 丸の内 (上書き)"); // no panic

	println!("{:?}", map);
	println!("{:?}", map["key-0"]);
	// println!("{:?}", map["key-1"]); // panic!
	println!("{:?}", map["key-2"]);
	println!("{:?}", map.get("undefined-key")); // no panic
}
