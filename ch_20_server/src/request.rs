pub struct Request {
	pub method: String,
	pub path: String,
}

impl Request {
	pub fn new() -> Request
	{
		Request {
			method: String::new(),
			path: String::new(),
		}
	}
}
