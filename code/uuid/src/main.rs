extern crate uuid;

use uuid::Uuid;

fn generate_uuid4() -> String {

	let uuid = Uuid::new_v4();
	let l1 = uuid.hyphenated().to_string();
	let l2 = format!("{}", uuid);
	assert_eq!(l1, l2);
	return l2;
}

fn generate_nil_uuid() -> String {

	let uuid = Uuid::nil();
	return uuid.hyphenated().to_string();
}

fn main() {

	let uuid = generate_uuid4();
	println!("uuid 4:\n    [{}]", uuid);

	let uuid = generate_uuid4();
	println!("uuid 4:\n    [{}]", uuid);

	let uuid = generate_uuid4();
	println!("uuid 4:\n    [{}]", uuid);

	let uuid = generate_nil_uuid();
	println!("空の UUID:\n    [{}]", uuid);
}
