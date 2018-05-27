extern crate uuid;

use uuid::Uuid;

fn generate_uuid4() -> String {

	let uuid = Uuid::new_v4();
	return format!("{}", uuid);
}

fn main() {

	let uuid = generate_uuid4();
	println!("{}", uuid);

	let uuid = generate_uuid4();
	println!("{}", uuid);

	let uuid = generate_uuid4();
	println!("{}", uuid);
}
