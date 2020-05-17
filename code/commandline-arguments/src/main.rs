extern crate clap;

mod configuration;

fn main() {
	let _conf = configuration::Configuration::configure();

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("what?");
		return;
	}

	for e in &args[1..] {
		println!("[{}]", e);
	}
}
