use std::io::Write;
use std::net::TcpStream;

pub fn respond(stream: &mut TcpStream, message: &str)
{
	let result = stream.write_all(message.as_bytes());
	if result.is_err() {
		eprintln!("ERROR: {:?} Failed to respond: {:?}", stream.peer_addr(), result.unwrap());
	}
}

pub fn respond_file(stream: &mut TcpStream, file_path: &str)
{
	let status_line = "HTTP/1.1 200 OK";
	let contents = std::fs::read_to_string(file_path).unwrap();
	let length = contents.len();

	let message = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

	respond(stream, &message);
}
