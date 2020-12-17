extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod test01;
mod test02;

fn run_tests() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	if true {
		test01::test0();
		test01::test1()?;
	}

	if false {
		test02::test02();
	}

	return Ok(());
}

fn main() {
	let result = run_tests();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] {}", error);
		return;
	}

	println!("Ok.");
}
