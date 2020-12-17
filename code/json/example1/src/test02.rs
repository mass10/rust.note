#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct User {
	email: String,
	name: String,
	birth: String,
}

pub fn test02() {
	extern crate serde;

	#[allow(unused)]
	fn parse_user_json(s: &str) -> std::result::Result<User, std::boxed::Box<dyn std::error::Error>> {
		let mut user = User {
			email: "".to_string(),
			name: "".to_string(),
			birth: "".to_string(),
		};
		let result = serde_json::from_str::<User>(s);

		return Ok(user);
	}
}
