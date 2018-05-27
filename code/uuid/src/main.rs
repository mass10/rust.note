extern crate uuid;

use uuid::Uuid;

fn generate_uuid4() -> String {

	let uuid = Uuid::new_v4();
	let l1 = uuid.hyphenated().to_string();
	let l2 = format!("{}", uuid);
	assert_eq!(l1, l2);
	return l2;
}

fn main() {

	let uuid = generate_uuid4();
	println!("{}", uuid);

	let uuid = generate_uuid4();
	println!("{}", uuid);

	let uuid = generate_uuid4();
	println!("{}", uuid);

	// all zero
	let uuid = Uuid::nil();
	println!("{}", uuid.hyphenated().to_string());
}
