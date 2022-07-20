use std::io::Write;
use std::net::TcpStream;

use crate::utils::asset::Asset;

pub fn send_page(mut stream: TcpStream, path: String) {
    let url = if path == "" { "index.html" } else { &path };
    let data = Asset::get(url).unwrap().data;
    let mut buf = Vec::new();
    let guess = mime_guess::from_path(url).first_or_octet_stream();
    for i in format!(
        "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\nContent-Type: {}/{}\r\n\r\n",
        data.len(),
        guess.type_(),
        guess.subtype()
    )
        .bytes()
    {
        buf.push(i);
    }
    for i in data.iter() {
        buf.push(*i);
    }
    stream.write(&buf).unwrap();
}
