fn main() {

	let v = vec![11, 22, 33, 44, 55];
	for e in v.into_iter().map(|x| x * 2) {
		println!("{:?}", e);
	}

	let v = vec![11, 22, 33, 44, 55];
	for e in v.into_iter() {
		println!("{:?}", e);
	}
}
