fn main() {

	{
		let separator = " ";
		let concat = |left, right| format!("{}{}{} さん", left, separator, right);
		println!("{}", concat("武田", "鉄矢"));
		println!("{}", concat("坂本", "竜馬"));
		println!("{}", concat("Hendrix", "Jimi"));
	}

	{
		let consumption_tax = 0.08;
		let calculate = |price| (price + (price * consumption_tax));
		println!("{}", calculate(100.0));
	}
}
