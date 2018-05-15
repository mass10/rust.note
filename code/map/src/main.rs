fn main() {

	// v を繰り返し e で受け取る
	let v = vec![11, 22, 33, 44, 55];
	for e in v.into_iter() {
		println!("{:?}", e);
	}

	// v を x * 2 したものを繰り返し e で受け取る
	let v = vec![11, 22, 33, 44, 55];
	for e in v.into_iter().map(|x| x * 2) {
		println!("{:?}", e);
	}
}
