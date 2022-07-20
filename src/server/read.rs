use std::io::Read;
use std::net::TcpStream;

pub fn read(mut stream: &TcpStream) -> String {
    let mut total = Vec::new();
    let mut buffer = [0; 4096];
    while stream.read(&mut buffer).unwrap() == 4096 {
        for i in buffer {
            total.push(i);
        }
    }
    for i in buffer {
        total.push(i);
    }
    String::from_utf8_lossy(&total).to_string()
}
