# コレクションの練習

# 配列の先頭とその他を切断するには

```Rust
	// 配列を配列に分割します。
	{
		let (head, latter) = ["a", "b", "c"].split_at(1);

		assert_eq!(head, &["a"]);
		assert_eq!(latter, &["b", "c"]);
	}

	// 先頭要素と配列に分割します。
	{
		let (head, latter) = ["a", "b", "c"].split_first().unwrap();

		assert_eq!(head, &"a");
		assert_eq!(latter, &["b", "c"]);
	}
```
