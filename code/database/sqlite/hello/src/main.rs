extern crate sqlite;

fn init() {

}

fn main() {

	let connection = sqlite::open(":memory:").unwrap();
    println!("Ok.");
}
