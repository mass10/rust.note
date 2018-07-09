fn main() {

	{
		println!("--- 1 ---");
		let values = std::ops::Range{start: 0, end: 10};
		for e in values {
			println!("[{:?}]", e);
		}
	}

	{
		println!("--- 2 ---");
		for e in 0..10 {
			println!("[{:?}]", e);
		}
	}

	{
		let left = std::ops::Range{start: 0, end: 10};
		let right = 0..10;
		assert!(left == right);
		assert_eq!(left, right);
	}
}
