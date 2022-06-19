# コレクションの練習

# 配列の先頭とその他を切断するには

```Rust
	let items = &["a", "b", "c"];
	let (head, latter) = items.split_first().unwrap();
```
