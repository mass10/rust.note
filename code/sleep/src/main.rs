/// 数値変換
fn parse_uint(s: &str) -> u64 {
	match s.trim().parse::<u64>() {
		Ok(n) => n,
		_ => 0,
	}
}

/// スリープ
fn sleep(secs: u64) {
	std::thread::sleep(std::time::Duration::from_secs(secs));
}

/// エントリーポイント
fn main() {
	// コマンドライン引数をひとつとる
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() < 1 {
		return;
	}
	// 数値変換
	let wait = parse_uint(&args[0]);
	if wait == 0 {
		return;
	}
	// スリープ
	sleep(wait);
}
