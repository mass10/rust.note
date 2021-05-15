/// 曜日 `n` の表示用テキストを返します。
///
/// # Arguments
/// `n` 任意の数値
///
/// # Returns
/// 曜日を表す文字列
fn name_of_day(n: i64) -> &'static str {
	return match n {
		0 => "Sunday",
		1 => "Monday",
		2 => "Tue",
		3 => "Wed",
		4 => "Thu",
		5 => "Fri",
		6 => "Sat",
		_ => "Eight Days A Week ?",
	};
}

fn name_of_day2(n: i32) -> &'static str {
	return match n {
		0..=9 => "0",
		10..=19 => "10",
		20..=29 => "20",
		30..=39 => "30",
		40..=49 => "40",
		50..=59 => "50",
		60..=69 => "60",
		70..=79 => "70",
		80..=89 => "80",
		90..=99 => "90",
		100..=109 => "100",
		_ => "",
	};
}

/// エントリーポイント
fn main() {
	// Eight days a week
	{
		println!("[trace] Eight days a week");
		for n in 0..8 {
			println!("{} => \"{}\"", n, name_of_day(n));
		}
	}

	// 範囲でマッチング
	{
		println!("[trace] 範囲でマッチング");
		assert_eq!("0", name_of_day2(0));
		assert_eq!("0", name_of_day2(9));
		assert_eq!("10", name_of_day2(10));
		assert_eq!("30", name_of_day2(39));
		assert_eq!("50", name_of_day2(52));
		assert_eq!("60", name_of_day2(69));
	}
}
