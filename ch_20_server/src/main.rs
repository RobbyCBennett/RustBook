mod limits;
mod handle;

use std::net::TcpListener;

use crate::handle::*;

fn main()
{
	let listener = TcpListener::bind("localhost:8080").unwrap();

	for result in listener.incoming() {
		if !result.is_err() {
			handle(result.unwrap());
		}
	}
}
