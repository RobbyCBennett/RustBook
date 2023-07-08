use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;

use crate::limits::*;

pub fn handle(mut stream: TcpStream)
{
	let buf_reader = BufReader::with_capacity(REQ_BYTES, &mut stream);

	dbg!("Request:");
	for line_result in buf_reader.lines() {
		if line_result.is_err() {
			dbg!("Failed to read this line");
			break;
		}

		let line = line_result.unwrap();
		dbg!("    {line}");
	}
}
