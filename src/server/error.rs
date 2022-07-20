use std::io::Write;
use std::net::TcpStream;

use crate::server::messaging_platform::messaging_platform;
use crate::utils::asset::Asset;

pub fn error(mut stream: TcpStream, code: i32, string_code: &str, response: String) {
    let sends = String::from_utf8_lossy(&Asset::get("error.html").unwrap().data.into_owned())
        .replace("$ERROR_CODE", &code.to_string());
    let messaging_platform = messaging_platform(response);
    stream
        .write(
            format!(
                "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                if messaging_platform { 200 } else { code },
                if messaging_platform {
                    "Ok"
                } else {
                    string_code
                },
                sends.len(),
                sends
            )
                .as_ref(),
        )
        .unwrap();
}
