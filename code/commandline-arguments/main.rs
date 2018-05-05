use std::env;

fn main() {

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("what?");
		return;
	}

	for e in &args[1..] {
	    println!("{:?}", e);
	}
}
