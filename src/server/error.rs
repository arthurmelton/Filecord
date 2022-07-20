use std::io::Write;
use std::net::TcpStream;

pub fn error(mut stream: TcpStream, code: i32, string_code: &str) {
    stream.write(format!("HTTP/1.1 {} {}\r\n\r\n", code, string_code).as_ref()).unwrap();
}