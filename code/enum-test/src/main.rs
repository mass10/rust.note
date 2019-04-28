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

fn diagnose(s: String) -> Option<Category> {
	return match s.as_str() {
		"Louis Armstrong" => Some(Category::Musician),
		_ => None,
	}
}

fn debug(s: String) {
	let category = diagnose(s.clone());
	if category.is_none() {
		println!("[TRACE] {} is None", s.clone());
		return;
	}
	println!("[TRACE] {} is {}", s, category.unwrap());
}

fn main() {
	debug("Louis Armstrong".to_owned());
	debug("Genpaku Sugita".to_owned());
}
