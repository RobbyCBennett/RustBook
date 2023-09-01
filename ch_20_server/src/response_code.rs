use std::fmt::{Debug, Display, Formatter, Result};

#[repr(u16)]
#[derive(Clone)]
pub enum ResponseCode {
	Good       = 200,
	Bad        = 400,
	BigRequest = 413,
	BigHeader  = 431,
}

impl ResponseCode {
	fn to_u16(&self) -> u16
	{
		self.clone() as u16
	}
}

impl Display for ResponseCode {
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "{}", self.to_u16())
	}
}

impl Debug for ResponseCode {
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "{}", match f.alternate() {
			false => self.to_u16().to_string(),
			true  => String::from(match self {
				ResponseCode::Good       => "Ok",
				ResponseCode::Bad        => "Bad Request",
				ResponseCode::BigRequest => "Payload Too Large",
				ResponseCode::BigHeader  => "Request Header Fields Too Large",
			}),
		})
	}
}
