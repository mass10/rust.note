extern crate csv;

fn read_main<R: std::io::Read>(stream: R) {

	let mut reader = csv::Reader::from_reader(stream);
	for row in reader.records() {
		if row.is_err() {
			continue;
		}
		let record = row.unwrap();
		println!("date: {:?}, mail: {:?}, name: {:?}",
			record[0].to_string(), record[1].to_string(), record[2].to_string());
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
