mod util;

use util::Stopwatch;

#[allow(unused)]
#[derive(Debug, Clone)]
struct MyRecord {
	id: String,
	email: String,
	name: String,
}

struct DataService {
	pub records: std::collections::LinkedList<MyRecord>,
}

impl DataService {
	pub fn new() -> Self {
		return DataService { records: Self::enum_records() };
	}

	fn enum_records() -> std::collections::LinkedList<MyRecord> {
		let mut list: std::collections::LinkedList<MyRecord> = std::collections::LinkedList::new();

		for i in 0..100000 {
			let record = MyRecord {
				id: format!("{}", i),
				email: format!("user-{}@gmail.com", i),
				name: format!("user-{}", i),
			};

			list.push_back(record);
		}

		return list;
	}
}

type MyTestCase = fn(service: &DataService);

/// テストケースを実行します。
///
/// # Arguments
/// * `test_case` テストケース
/// * `try_count` 試行回数
/// * `records` テストデータ
fn run_test_case(label: &str, test_case: MyTestCase, try_count: u32, service: &DataService) {
	println!("[INFO] START: [{}]", label);

	// 時間集約用
	let mut set = std::collections::BTreeSet::<String>::new();

	for _ in 0..try_count {
		let stopwatch = Stopwatch::new();
		test_case(&service);
		set.insert(stopwatch.to_string());
	}

	println!("処理時間: {:?}", set);
}

/// レコードの検証(ダミー)
fn validate_record(record: &MyRecord) {
	if record.id.contains(".") {
		println!("[ERROR] {:?}", &record);
	}
}

/// CASE:A コピーによるイテレーション操作
fn run_test_a(service: &DataService) {
	// レコードのイテレーション
	for record in service.records {
		validate_record(&record);
	}
}

/// CASE:B 参照によるイテレーション操作
fn run_test_b(service: &DataService) {
	// レコードのイテレーション
	for record in service.records.iter() {
		validate_record(&record);
	}
}

/// エントリーポイント
fn main() {
	let service = DataService::new();

	const TRY_COUNT: u32 = 1000;

	run_test_case("CASE:A コピーによるイテレーション操作", run_test_a, TRY_COUNT, &service);
	run_test_case("CASE:B 参照によるイテレーション操作", run_test_b, TRY_COUNT, &service);
}
