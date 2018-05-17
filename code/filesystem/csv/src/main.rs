extern crate csv;

fn read_main<R: std::io::Read>(stream: R) {

	let mut csv_reader = csv::Reader::from_reader(stream);
	for row in csv_reader.records() {
		if row.is_err() {
			continue;
		}
		let row = row.unwrap();
		println!("{:?}", row);
	}
}

fn main() {

	if false {
		let path = std::path::Path::new("sample.csv");
		let file = std::fs::File::open(path).unwrap();
		read_main(file);
	}

	if true {
		read_main(std::io::stdin());
	}
}
