fn main() {
	// コマンドライン引数の取り出し
	{
		println!("[TRACE] ========== コマンドライン引数のテスト ==========");
		let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
		println!("[TRACE] 引数の数は {}", args.len());
		for e in args {
			println!("[TRACE] [{}]", e);
		}
		println!();
	}

	{
		println!("[TRACE] ========== std::vec::Vec<&str> のテスト ==========");
		// let mut list: std::vec::Vec<&str> = vec![];
		let mut list = std::vec::Vec::new();
		list.push("まどか");
		list.push("さやか");
		list.push("杏子");
		list.push("マミさん");
		list.push("ほむ");
		println!("[TRACE] {:?}", list);
		println!();
	}

	{
		println!("[TRACE] ========== std::vec::Vec<String> のテスト ==========");
		let mut list: std::vec::Vec<String> = vec!();
		list.push(String::from("まどか"));
		list.push(String::from("杏子"));
		list.push(String::from("マミさん"));
		list.push(String::from("さやか"));
		list.push(String::from("ほむ"));
		println!("[TRACE] {:?}", list);
		println!();
	}
}