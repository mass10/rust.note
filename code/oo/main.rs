#[derive(Debug)]
struct Application {
	id: String
}

impl Application {
	fn run(&self) {
		println!("### START ###");
		println!("{}", self.get_id());
		println!("--- END ---");
	}
	fn get_id(&self) -> String {
		return self.id.to_string();
	}
}

fn main() {
	let app = Application {
		id: String::from("ABC")
	};
	app.run();
}
