fn main() {
	// コマンドライン引数の取り出し
	{
		println!("[TRACE] ========== コマンドライン引数の操作 ==========");
		let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
		println!("[TRACE] 引数の数は {}", args.len());
		for e in args {
			println!("[TRACE] [{}]", e);
		}
		println!();
	}

	{
		println!("[TRACE] ========== 固定長配列 の操作 ==========");

		let mut array_container = [0, 2, 27, 82, 102, 255];

		array_container[3] = 0;
		array_container[4] = 0;
		// array_container[6] = 0; // コンパイルエラー

		for e in &array_container {
			println!("[TRACE] [{}]", &e);
		}

		// 配列を分割([先頭], [その他, その他, その他, ...])
		{
			let (head, latter) = [1, 2, 3].split_at(1);

			println!("[TRACE] head: {:?}, latter: {:?}", head, latter);

			assert_eq!(head, &[1]);
			assert_eq!(latter, &[2, 3]);
		}

		{
			let (head, latter) = ["a", "b", "c"].split_at(1);

			println!("[TRACE] head: {:?}, latter: {:?}", head, latter);

			assert_eq!(head, &["a"]);
			assert_eq!(latter, &["b", "c"]);
		}

		// 配列を分割(先頭, [その他, その他, その他, ...])
		{
			let (head, latter) = [1, 2, 3].split_first().unwrap();

			println!("[TRACE] head: {:?}, latter: {:?}", head, latter);

			assert_eq!(head, &1);
			assert_eq!(latter, &[2, 3]);
		}

		{
			let (head, latter) = ["a", "b", "c"].split_first().unwrap();

			assert_eq!(head, &"a");
			assert_eq!(latter, &["b", "c"]);
		}

		println!();
	}

	{
		println!("[TRACE] ========== LinkedList<String> の操作 ==========");

		// ・コレクションの内容は挿入された順序を保持します。

		let mut list: std::collections::LinkedList<String> = std::collections::LinkedList::new();

		list.push_back("博多ラーメン".to_string());
		list.push_back("のりラーメン".to_string());
		list.push_back("ワンタンメン".to_string());

		for e in &list {
			println!("[TRACE] [{}]", &e);
		}

		println!();
	}

	if false {
		println!("[TRACE] ========== std::vec::Vec<&str> の操作 ==========");

		// ・コレクションの内容は挿入された順序を保持します。

		let mut list: std::vec::Vec<&str> = vec![];

		list.push("まどか");
		list.push("さやか");
		list.push("杏子");
		list.push("マミさん");
		list.push("ほむ");

		for e in list {
			println!("[TRACE] {}", &e);
		}

		println!();
	}

	if false {
		println!("[TRACE] ========== std::vec::Vec<String> の操作 ==========");

		// ・コレクションの内容は挿入された順序を保持します。

		let mut list: std::vec::Vec<String> = vec![];

		list.push(String::from("まどか"));
		list.push(String::from("杏子"));
		list.push(String::from("マミさん"));
		list.push(String::from("さやか"));
		list.push(String::from("ほむ"));

		for e in list {
			println!("[TRACE] {}", &e);
		}

		println!();
	}

	if false {
		println!("[TRACE] ========== std::collections::HashMap<String, i8> の操作 ==========");

		// ・コレクションの内容はソートされません。

		let mut map: std::collections::HashMap<String, i8> = std::collections::HashMap::new();

		map.insert(String::from("まどか"), 13);
		map.insert(String::from("杏子"), 13);
		map.insert(String::from("マミさん"), 15);
		map.insert(String::from("さやか"), 13);
		map.insert(String::from("ほむ"), 13);

		println!("[TRACE] {}", map.contains_key("さやか"));

		for (k, v) in &map {
			println!("[TRACE] KEY: [{}], VALUE: [{}]", k, v);
		}

		println!("[TRACE] {:?}", map);

		println!();
	}

	if false {
		println!("[TRACE] ========== std::collections::HashMap<&str, i8> の操作 ==========");

		// ・コレクションの内容はソートされません。

		let mut map: std::collections::HashMap<&str, i8> = std::collections::HashMap::new();

		map.insert("まどか", 13);
		map.insert("杏子", 13);
		map.insert("マミさん", 15);
		map.insert("さやか", 13);
		map.insert("ほむ", 13);

		println!("[TRACE] {}", map.contains_key("さやか"));

		for (k, v) in &map {
			println!("[TRACE] KEY: [{}], VALUE: [{}]", k, v);
		}

		println!("[TRACE] {:?}", map);

		println!();
	}

	if false {
		println!("[TRACE] ========== std::collections::BTreeMap<String, i8> の操作 ==========");

		// ・コレクションの内容はソートされます。

		let mut map: std::collections::BTreeMap<String, i8> = std::collections::BTreeMap::new();

		map.insert(String::from("まどか"), 13);
		map.insert(String::from("杏子"), 13);
		map.insert(String::from("マミさん"), 15);
		map.insert(String::from("さやか"), 13);
		map.insert(String::from("ほむ"), 13);

		println!("[TRACE] {}", map.contains_key("さやか"));

		for (k, v) in &map {
			println!("[TRACE] KEY: [{}], VALUE: [{}]", k, v);
		}

		println!("[TRACE] {:?}", map);

		println!();
	}

	if false {
		println!("[TRACE] ========== std::collections::HashSet<&str> の操作 ==========");

		// ・コレクションの内容はソートされません。

		let mut set: std::collections::HashSet<&str> = std::collections::HashSet::new();

		set.insert("まどか");
		set.insert("杏子");
		set.insert("マミさん");
		set.insert("さやか");
		set.insert("ほむ");

		println!("[TRACE] {}", set.contains("さやかちゃん"));

		for e in &set {
			println!("[TRACE] ENTRY: [{}]", e);
		}

		println!("[TRACE] {:?}", set);

		println!();
	}
}
