fn main() {

	println!("Hello, world!");

	println!("{}", "あいうえお");
	println!("{}, {}, {}", 1, "2", '3');

	println!("{}", 95);
	println!("{:X}", 95);
	println!("{:#X}", 95);
	println!("{:x}", 95);
	println!("{:#x}", 95);
	println!("{:08x}", 65536);
	println!("{:#010x}", 65536);
	println!("{:b}", 95);
	println!("{:#b}", 95);

	println!("family name: [{family_name}], first name: [{first_name}]", first_name="John", family_name="Jones");
	println!("都道府県: [{都道府県}], 市区町村: [{市区町村}]", 市区町村="小樽市南台2-1", 都道府県="北海道");
}
