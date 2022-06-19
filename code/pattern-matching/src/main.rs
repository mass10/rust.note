/// 年齢 age を文字列に変換します。
fn to_string(age: i64) -> String {
	if age < 0 {
		return String::from("");
	}

	// range によるパターンマッチングです。
	// [2] が推奨となりました。以前の [1] は deprecated です。
	// [1] L...R ※deprecated
	// [2] L..=R
	match age {
		0..=19 => String::from("10代"),
		20..=29 => String::from("20代"),
		30..=39 => String::from("30代"),
		40..=49 => String::from("40代"),
		50..=59 => String::from("50代"),
		60..=69 => String::from("60代"),
		70..=79 => String::from("70代"),
		80..=89 => String::from("80代"),
		_ => String::from("90歳以上"),
	}
}

/// 年齢 age を説明します。
fn describe_age(age: i64) {
	let description = to_string(age);
	println!("{} 歳は {:?} です。", age, description);
}

/// エントリーポイントです。
fn main() {
	describe_age(-123);
	describe_age(-1);
	describe_age(0);
	describe_age(1);
	describe_age(12);
	describe_age(19);
	describe_age(20);
	describe_age(21);
	describe_age(59);
	describe_age(90);
	describe_age(100);
	describe_age(300);
}
