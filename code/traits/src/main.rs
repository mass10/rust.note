mod case_integral;
mod case_string;
mod case_string2;
mod case_vector;
mod myutil;

fn main() {
	case_integral::execute();
	case_string::execute();
	case_string2::execute();
	case_vector::execute();
	println!("Ok.");
}
