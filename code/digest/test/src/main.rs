//
//
//
//
//
// 【考察】
// ・待機を入れるとかなり遅くなるのは何故か。
//   std::thread::sleep(std::time::Duration::from_millis()) にかなりのオーバーヘッドがある？
//
//
//
//

mod stopwatch;

/// md5
fn digest_md5(s: &str) -> String {
	return format!("{:x}", md5::compute(s));
}

/// crypto::sha2::Sha256
fn digest_sha256_0(s: &str) -> String {
	extern crate crypto;
	use crypto::digest::Digest;

	let mut digest = crypto::sha2::Sha256::new();
	digest.input_str(s);
	return format!("{}", digest.result_str());
}

/// sha2::Sha256
fn digest_sha256_1(s: &str) -> String {
	use sha2::Digest;

	let mut digest = sha2::Sha256::new();
	digest.update(s);
	let result = digest.finalize();
	return format!("{:x}", result);
}

fn calculate_time_md5(s: &str, try_count: u32) {
	let stopwatch = stopwatch::Stopwatch::new();
	for _ in 1..try_count {
		// md5
		let digest = digest_md5(s);
		assert_ne!(digest, "");
		// 待機を入れると想像よりもはるかに遅くなるのは何故か...
		// std::thread::sleep(std::time::Duration::from_millis(1));
	}
	println!("[TRACE] md5                  ... [{}]", stopwatch);
}

fn calculate_time_sha256_0(s: &str, try_count: u32) {
	let stopwatch = stopwatch::Stopwatch::new();
	for _ in 1..try_count {
		// crypto::sha2::Sha256
		let digest = digest_sha256_0(s);
		assert_ne!(digest, "");
		// 待機を入れると想像よりもはるかに遅くなるのは何故か...
		// std::thread::sleep(std::time::Duration::from_millis(1));
	}
	println!("[TRACE] crypto::sha2::Sha256 ... [{}]", stopwatch);
}

fn calculate_time_sha256_1(s: &str, try_count: u32) {
	let stopwatch = stopwatch::Stopwatch::new();
	for _ in 1..try_count {
		// sha2::Sha256
		let digest = digest_sha256_1(s);
		assert_ne!(digest, "");
		// 待機を入れると想像よりもはるかに遅くなるのは何故か...
		// std::thread::sleep(std::time::Duration::from_millis(1));
	}
	println!("[TRACE] sha2::Sha256         ... [{}]", stopwatch);
}

fn main() {
	calculate_time_md5("Hello, world!", 10000);
	calculate_time_sha256_0("Hello, world!", 10000);
	calculate_time_sha256_1("Hello, world!", 10000);
}
