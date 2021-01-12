fn add(base: i32) -> impl Fn(i32) -> i32 {
	let f = move |n| {
		return base * n;
	};
	return f;
}

fn main() {
	println!("{}", add(1)(2)); // 2
	println!("{}", add(2)(3)); // 6
	println!("{}", add(3)(4)); // 12
}
