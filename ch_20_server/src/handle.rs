use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;

use crate::limits::*;
use crate::respond::*;

fn sleep()
{
	std::thread::sleep(std::time::Duration::from_secs(5));
}

macro_rules! response {
	($code:ident) =>
	{
		format!("HTTP/1.1 {:?} {:#?}\r\n\r\n", $code, $code)
	};
	($code:ident, $content:ident) =>
	{
		format!("HTTP/1.1 {:?} {:#?}\r\nContent-Length: {}\r\n\r\n{}",
			$code, $code, $content.len(), $content
		)
	}
}

const FILE_INDEX: &str = "public/index.html";
const FILE_404:   &str = "public/404.html";

struct Request {
	method: String,
	path: String,
}

impl Request {
	fn new() -> Request
	{
		Request {
			method: String::new(),
			path: String::new(),
		}
	}
}

#[repr(u16)]
#[derive(Clone)]
enum ResponseCode {
	Good = 200,
	Bad = 400,
	BigHeader = 431,
}

impl ResponseCode {
	fn to_u16(&self) -> u16
	{
		self.clone() as u16
	}
}

impl std::fmt::Display for ResponseCode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{}", self.to_u16())
	}
}

impl std::fmt::Debug for ResponseCode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{}", match f.alternate() {
			false => self.to_u16().to_string(),
			true  => String::from(match self {
				ResponseCode::Good      => "Ok",
				ResponseCode::Bad       => "Bad Request",
				ResponseCode::BigHeader => "Request Header Fields Too Large",
			}),
		})
	}
}

struct RequestError {
	code: ResponseCode,
	address: std::net::SocketAddr,
}

impl RequestError {
	fn new(code: ResponseCode, request: &std::net::TcpStream) -> RequestError
	{
		RequestError {
			code,
			address: request.peer_addr().unwrap(),
		}
	}
}

impl std::fmt::Debug for RequestError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "ERROR: {} {:#?}", self.address, self.code)
	}
}

pub fn handle(mut stream: TcpStream)
{
	match parse_request(&stream) {
		Ok(request)        => handle_good_request(&mut stream, request),
		Err(request_error) => handle_bad_request(&mut stream, request_error),
	}
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

fn handle_bad_request(stream: &mut TcpStream, request_error: RequestError)
{
	// Log error
	eprintln!("{:?}", request_error);

	// Respond
	let code = request_error.code;
	respond(stream, &response!(code));
}

fn parse_request(stream: &TcpStream) -> Result<Request, RequestError>
{
	let mut request = Request::new();

	// Parse each line of the request
	let buf_reader = BufReader::with_capacity(REQ_BUF_BYTES, stream);
	let mut byte_count: usize = 0;
	for (i, line_result) in buf_reader.lines().enumerate() {
		// Get line or error
		if line_result.is_err() {
			return Err(RequestError::new(ResponseCode::Bad, stream));
		}
		let line = line_result.unwrap();

		// Add to byte count or error if request is too large
		let length = line.len();
		let add_result = byte_count.checked_add(length);
		if add_result == None || add_result.unwrap() > MAX_REQ_HEADER_BYTES {
			return Err(RequestError::new(ResponseCode::BigHeader, stream));
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
