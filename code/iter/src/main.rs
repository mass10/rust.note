fn main() {
	let mut it = core::iter::once(5);

	while let Some(x) = it.next() {
		println!("[TRACE] ({})", x);
	}
}
