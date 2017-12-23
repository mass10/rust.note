fn test_for() {
	
	for i in 0..99 {
		println!("{0}", i);
	}
}

fn test_while() {

	let mut i = 0;
	while i < 100 {
		println!("{}", i);
		i += 1;
	}

	i = 0;
	loop {
		if 100 <= i {
			break;
		}
		println!("{}", i);
		i += 1;
	}
}

fn main() {

	test_for();
	test_while();
}

