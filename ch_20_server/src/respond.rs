use std::io::Write;
use std::net::TcpStream;

use crate::response_code::*;

pub fn response(code: ResponseCode) -> String
{
	format!("HTTP/1.1 {:?} {:#?}\r\n\r\n", code, code)
}

pub fn response_content(code: ResponseCode, content: &str) -> String
{
	format!("HTTP/1.1 {:?} {:#?}\r\nContent-Length: {}\r\n\r\n{}",
		code, code, content.len(), content)
}

pub fn respond(stream: &mut TcpStream, message: &str)
{
	let result = stream.write_all(message.as_bytes());
	if result.is_err() {
		eprintln!("ERROR: {:?} Failed to send response: {:?}", stream.peer_addr(), result.unwrap());
	}
}

pub fn respond_file(stream: &mut TcpStream, file_path: &str)
{
	let code = ResponseCode::Good;
	let content = std::fs::read_to_string(file_path).unwrap();

	respond(stream, &response_content(code, &content));
}
