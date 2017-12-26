#[derive(Debug)]
struct Something {
	id: String
}

impl Something {
	fn test1(&self) -> String {
		return self.id.to_string();
	}	
}

fn main() {
	let unknwon = Something{
		id: String::from("ABC")
	};
	println!("{}", unknwon.test1());
}
