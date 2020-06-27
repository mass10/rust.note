extern crate md5;

fn digest_md5(s: &str) -> String {
	return format!("{:x}", md5::compute(s));
}

fn test(s: &str) {
	let digest = digest_md5(s);
	println!("[TRACE] [{}] >> [{}] (MD5)", s, digest);
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
