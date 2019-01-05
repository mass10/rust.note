trait TestableNumber<T> {
	fn is_a(&self) -> bool;
	fn value(&self) -> T;
}

trait TestableString<T> {
	fn is_alpha(&self) -> bool;
	fn value(&self) -> T;
}

impl TestableNumber<i32> for i32 {
	fn is_a(&self) -> bool {
		match *self {
			0x41 => true,
			0x61 => true,
			_ => false,
		}
	}
	fn value(&self) -> i32 {
		return *self;
	}
}

impl TestableString<String> for String {
	fn is_alpha(&self) -> bool {
		if self.len() == 0 {
			return false;
		}
		for ch in self.chars() {
			if ch.is_uppercase() {
				continue;
			}
			if ch.is_lowercase() {
				continue;
			}
			return false;
		}
		return true;
	}
	fn value(&self) -> String {
		return self.clone();
	}
}

impl TestableString<&'static str> for &'static str {
	fn is_alpha(&self) -> bool {
		if self.len() == 0 {
			return false;
		}
		for ch in self.chars() {
			if ch.is_uppercase() {
				continue;
			}
			if ch.is_lowercase() {
				continue;
			}
			return false;
		}
		return true;
	}
	fn value(&self) -> &'static str {
		return *self;
	}
}

fn test_integral_value(n: &TestableNumber<i32>) {
	println!("{} -> {:?}", n.value(), n.is_a());
}

fn test_string_value(n: &TestableString<String>) {
	println!("{:?}", n.is_alpha());
}

fn main() {

	{
		println!("### 数値をテストします ###");
		for n in 0..100 {
			test_integral_value(&n);
		}
	}

	{
		println!("### 文字列をテストします ###");
		test_string_value(&String::from("くぃうえお"));
	}

	println!("{:?}", String::new());
	println!("Ok.");
}
