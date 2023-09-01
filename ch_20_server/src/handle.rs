use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::time::Duration;

use crate::config::*;
use crate::request::*;
use crate::request_error::*;
use crate::respond::*;
use crate::response_code::*;

pub fn handle(mut stream: TcpStream)
{
	match parse_request(&stream) {
		Err(request_error) => handle_bad_request(&mut stream, request_error),
		Ok(request)        => handle_good_request(&mut stream, request),
	}
}

fn parse_request(stream: &TcpStream) -> Result<Request, RequestError>
{
	let mut request = Request::new();

	// Parse each line of the request
	let buf_reader = BufReader::with_capacity(MAX_REQ_BYTES, stream);
	let mut byte_count: usize = 0;
	for (i, line_result) in buf_reader.lines().enumerate() {
		// Get line or error
		if line_result.is_err() {
			return Err(RequestError::new(ResponseCode::Bad, stream));
		}
		let line = line_result.unwrap();

		// Error if header is too large
		if line.len() > MAX_REQ_HEADER_BYTES {
			return Err(RequestError::new(ResponseCode::BigHeader, stream));
		}

		// Add to byte count or error if request is too large
		let add_result = byte_count.checked_add(line.len());
		if add_result == None || add_result.unwrap() > MAX_REQ_BYTES {
			return Err(RequestError::new(ResponseCode::BigRequest, stream));
		}
		byte_count = add_result.unwrap();

		// Parse first line
		if i == 0 {
			// Split by whitespace
			for (j, word) in line.split_whitespace().enumerate() {
				if j == 0 {
					request.method = word.to_string();
				}
				else {
					request.path = word.to_string();
					break;
				}
			}
			// Stop parsing message early if method is GET
			if request.method == "GET" {
				break;
			}
			// Error if no path
			if request.path.is_empty() {
				return Err(RequestError::new(ResponseCode::Bad, stream));
			}
		}

		// Stop reading the request
		if line.is_empty() {
			break;
		}
	}

	return Ok(request);
}

fn handle_bad_request(stream: &mut TcpStream, request_error: RequestError)
{
	// Log error
	eprintln!("{:?}", request_error);

	respond(stream, &response(request_error.code));
}

fn handle_good_request(stream: &mut TcpStream, request: Request)
{
	// Get file path, waiting if the request is /sleep
	let file_path = match request.method.as_str() {
		"GET" => match request.path.as_str() {
			"/" | "/index" | "/index.html" => FILE_INDEX,
			"/sleep"                       => { sleep(); FILE_INDEX },
			_                              => FILE_404,
		},
		_ => FILE_404,
	};

	respond_file(stream, file_path);
}

fn sleep()
{
	std::thread::sleep(Duration::from_secs(SLEEP_SECONDS));
}
