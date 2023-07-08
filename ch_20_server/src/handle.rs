use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
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

pub fn respond(stream: &mut TcpStream, message: &str)
{
	let result = stream.write_all(message.as_bytes());
	if result.is_err() {
		eprintln!("ERROR: {:?} Failed to respond: {:?}", stream.peer_addr(), result.unwrap());
	}
}

pub fn respond_file(stream: &mut TcpStream, filepath: &str)
{
	let status_line = "HTTP/1.1 200 OK";
	let contents = fs::read_to_string(filepath).unwrap();
	let length = contents.len();

	let message = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

	respond(stream, &message);
}

pub fn handle(mut stream: TcpStream)
{
	let buf_reader = BufReader::with_capacity(MAX_REQ_BYTES, &stream);

	debug!("Request:");

	let mut byte_count: usize = 0;
	for line_result in buf_reader.lines() {
		// Get line or error
		if line_result.is_err() {
			eprintln!("ERROR: {:?} Failed to read this line", stream.peer_addr());
			return respond(&mut stream, "HTTP/1.1 400 Bad Request\r\n\r\n");
		}
		let line = line_result.unwrap();

		// Add to byte count or error if request is too large
		let length = line.len();
		let add_result = byte_count.checked_add(length);
		if add_result == None || add_result.unwrap() > MAX_REQ_BYTES {
			eprintln!("ERROR: {:?} Too many bytes", stream.peer_addr());
			return respond(&mut stream, "HTTP/1.1 431 Request Header Fields Too Large\r\n\r\n");
		}
		byte_count = add_result.unwrap();

		debug!("    {line}");

		// Stop reading the request
		if line.is_empty() {
			break;
		}
	}

	// return respond(&mut stream, "HTTP/1.1 200 OK\r\n\r\n");

	return respond_file(&mut stream, "public/index.html");
}
