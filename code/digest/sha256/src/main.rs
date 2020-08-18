fn digest_sha256_0(s: &str) -> String {
	extern crate crypto;
	use crypto::digest::Digest;

	let mut digest = crypto::sha2::Sha256::new();
	digest.input_str(s);
	return format!("{}", digest.result_str());
}

fn digest_sha256_1(s: &str) -> String {
	use sha2::Digest;

	let mut digest = sha2::Sha256::new();
	digest.update(s);
	let result = digest.finalize();
	return format!("{:x}", result);
}

fn test(s: &str) {
	let digest0 = digest_sha256_0(s);
	let digest1 = digest_sha256_1(s);
	assert_eq!(digest0, digest1);
	println!("[{}] >> [{}] (SHA256)", s, digest0);
}

fn main() {

	let argv: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if argv.len() != 0 {
		test(&*argv[0]);
		return;
	}

	test("Hello, world!");
	test("コンニチハ！");
	test("https://www.nittsu.co.jp");
	test("https://www.mazda.co.jp");
	test("https://www.mol.co.jp");
	test("https://www.nissan.co.jp");
	test("https://www.marubeni.com");
	test("https://www.hd.eneos.co.jp");
}
