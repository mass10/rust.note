pub struct Thread2 {
	sender: std::sync::mpsc::Sender<String>,
}

impl Thread2 {
	pub fn new() -> Self {
		let (sender, _receiver) = std::sync::mpsc::channel();
		Thread2 { sender }
	}

	pub fn run(&self) {
		self.sender.send("Hello from Thread2".to_string()).unwrap();
	}
}
