use std::io::{Write, Read};
use std::net::TcpStream;

pub fn send_to_user(filename: String, length: u64, ids: Vec<String>, channel: u64, mut stream: TcpStream) {
    stream.write(format!("HTTP/1.1 200 Ok\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n", filename, length).as_bytes()).unwrap();
    let mut index = 0;
    for id in &ids {
        let mut buffer = Vec::new();
        ureq::get(&format!(
            "https://cdn.discordapp.com/attachments/{}/{}/part_{}",
            channel, id, index
        ))
            .call()
            .unwrap()
            .into_reader()
            .read_to_end(&mut buffer)
            .unwrap();
        stream.write(&buffer).unwrap();
        index += 1;
    }
}