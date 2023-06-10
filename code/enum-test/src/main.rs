use std::fmt;

#[allow(dead_code)]
enum Category {
	Architect,
	Doctor,
	Lawyer,
	Musician,
	Politician,
	Programmer,
}

impl fmt::Display for Category {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Category::Architect => write!(f, "建築家"),
			Category::Lawyer => write!(f, "弁護士"),
			Category::Doctor => write!(f, "医師"),
			Category::Musician => write!(f, "音楽家"),
			Category::Politician => write!(f, "政治家"),
			Category::Programmer => write!(f, "プログラマー"),
		}		
	}
}

fn diagnose(s: &str) -> Option<Category> {
	return match s {
		"Louis Armstrong" => Some(Category::Musician),
		"Genpaku Sugita" => Some(Category::Doctor),
		_ => None,
	}
}

fn debug(s: &str) {
	let category = diagnose(s);
	if category.is_none() {
		println!("[TRACE] {} is None", s.clone());
		return;
	}
	println!("[TRACE] {} is {}", s, category.unwrap());
}

/// エントリーポイント
fn main() {
	debug("Louis Armstrong");
	debug("Genpaku Sugita");
}
