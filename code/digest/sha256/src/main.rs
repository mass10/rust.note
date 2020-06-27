extern crate crypto;

fn digest_sha256(s: &str) -> String {
	use crypto::digest::Digest;
	use crypto::sha2::Sha256;

	let mut digest = Sha256::new();
	digest.input_str(s);
	return format!("{}", digest.result_str());
}

fn test(s: &str) {
	let digest = digest_sha256(s);
	println!("[{}] >> [{}] (SHA256)", s, digest);
}

fn main() {
	test("Hello, world!");
	test("コンニチハ！");
	test("https://www.nittsu.co.jp");
	test("https://www.mazda.co.jp");
	test("https://www.mol.co.jp");
	test("https://www.nissan.co.jp");
	test("https://www.marubeni.com");
	test("https://www.hd.eneos.co.jp");
}
