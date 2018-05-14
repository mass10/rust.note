fn get_current_position() -> (i64, i64) {

	return (1, 1272);
}

fn main() {

	{
		let position = get_current_position();
		// shows (left, right)
		println!("{:?}", position);
		// shows left one.
		println!("{}", position.0);
		// shows right one.
		println!("{}", position.1);
		// immutable
		// position.0 = 0;
		// compilation fails.
		// println!("{:?}", position.2);
	}

	{
		let mut position = get_current_position();
		// mutable
		position.1 = position.1 + 100;
		println!("{:?}", position);
	}
}
