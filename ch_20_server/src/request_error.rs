use std::net::{SocketAddr, TcpStream};
use std::fmt::{Debug, Formatter, Result};

use crate::response_code::*;

pub struct RequestError {
	pub code: ResponseCode,
	pub address: SocketAddr,
}

impl RequestError {
	pub fn new(code: ResponseCode, request: &TcpStream) -> RequestError
	{
		RequestError {
			code,
			address: request.peer_addr().unwrap(),
		}
	}
}

impl Debug for RequestError {
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ERROR: {} {:#?}", self.address, self.code)
	}
}
