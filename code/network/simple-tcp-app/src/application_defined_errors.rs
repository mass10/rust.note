#[derive(Debug, Clone)]
pub struct ApplicationError {
	/// エラーの内容
	pub description: String,
}

impl ApplicationError {
	/// 新しいインスタンスを返します。
	pub fn new(description: &str) -> ApplicationError {
		return ApplicationError { description: description.to_string() };
	}
}

impl std::fmt::Display for ApplicationError {
	/// [std::fmt::Display] として振る舞うための既定の操作を提供します。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{}", self.description)
	}
}

impl std::error::Error for ApplicationError {
	/// [std::error::Error] として振る舞うための既定の操作を提供します。
	///
	/// # Returns エラーの内容
	fn description(&self) -> &str {
		&self.description
	}
}
