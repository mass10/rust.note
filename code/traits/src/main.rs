mod case_integral;
mod case_string;
mod case_string2;
mod case_vector;
mod myutil;

fn main() {

	// 数値に trait を実装してみる
	case_integral::execute();

	// 文字列に trait を実装してみる
	case_string::execute();

	// 文字列に trait を実装してみる
	case_string2::execute();

	// Vec に trait を実装してみる
	case_vector::execute();

	println!("Ok.");
}
