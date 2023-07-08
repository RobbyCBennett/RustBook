use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;

use crate::limits::*;
use crate::respond::*;

#[cfg(debug_assertions)]
macro_rules! debug {
	( $x:expr ) => { println!($x) };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
	($x:expr) => { std::convert::identity($x) }
}

const RES_BAD: &str        = "HTTP/1.1 400 Bad Request\r\n\r\n";
const RES_BIG_HEADER: &str = "HTTP/1.1 431 Request Header Fields Too Large\r\n\r\n";

const FILE_INDEX: &str = "public/index.html";
const FILE_404: &str   = "public/404.html";

pub fn handle(mut stream: TcpStream)
{
	let buf_reader = BufReader::with_capacity(REQ_BUF_BYTES, &stream);

	// Create request info
	let mut method = String::new();
	let mut path   = String::new();

	// Parse request
	let mut byte_count: usize = 0;
	for (i, line_result) in buf_reader.lines().enumerate() {
		// Get line or error
		if line_result.is_err() {
			eprintln!("ERROR: {:?} Failed to read request", stream.peer_addr());
			return respond(&mut stream, RES_BAD);
		}
		let line = line_result.unwrap();

		// Add to byte count or error if request is too large
		let length = line.len();
		let add_result = byte_count.checked_add(length);
		if add_result == None || add_result.unwrap() > MAX_REQ_HEADER_BYTES {
			eprintln!("ERROR: {:?} Too many bytes", stream.peer_addr());
			return respond(&mut stream, RES_BIG_HEADER);
		}
		byte_count = add_result.unwrap();

		// Parse first line
		if i == 0 {
			// Split by whitespace
			for (j, word) in line.split_whitespace().enumerate() {
				if j == 0 {
					method = word.to_string();
				}
				else {
					path = word.to_string();
					break;
				}
			}
			// Stop parsing message early if method is GET
			if method == "GET" {
				break;
			}
			// Error if no path
			if path.is_empty() {
				eprintln!("ERROR: {:?} Failed to read request", stream.peer_addr());
				return respond(&mut stream, RES_BAD);
			}
		}

		// Stop reading the request
		if line.is_empty() {
			break;
		}
	}

	// Route
	let filename = match method.as_str() {
		"GET" => match path.as_str() {
			"/" | "/index" | "/index.html" => FILE_INDEX,
			_ => FILE_404,
		},
		_ => FILE_404,
	};

	return respond_file(&mut stream, filename);
}
