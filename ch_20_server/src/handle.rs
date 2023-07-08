use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;

use crate::limits::*;

#[cfg(debug_assertions)]
macro_rules! debug {
	( $x:expr ) => { println!($x) };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
	($x:expr) => { std::convert::identity($x) }
}

pub fn handle(mut stream: TcpStream)
{
	let buf_reader = BufReader::with_capacity(REQ_BYTES, &mut stream);

	debug!("Request:");

	let mut bytes: usize = 0;
	for line_result in buf_reader.lines() {
		// Get line or error
		if line_result.is_err() {
			eprintln!("ERROR: Failed to read this line");
			break;
		}
		let line = line_result.unwrap();

		// Add to bytes or error if request is too large
		let length = line.len();
		let add_result = bytes.checked_add(length);
		if add_result == None || add_result.unwrap() > REQ_BYTES {
			eprintln!("ERROR: Too many bytes");
			break;
		}
		bytes = add_result.unwrap();

		debug!("    {line}");
	}
}
