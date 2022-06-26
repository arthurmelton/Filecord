use flate2::write::ZlibDecoder;
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use urlencoding::decode;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", env::var("PORT").unwrap_or("80".to_string()))).unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();
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
            let response: String = String::from_utf8_lossy(&total).to_string();
            let url = response.split(" ").nth(1).unwrap().chars().collect::<Vec<char>>()[1..].into_iter().collect::<String>().trim().to_string();
            let mut writer = Vec::new();
            let mut z = ZlibDecoder::new(writer);
            z.write_all(&base64::decode(&url).unwrap()).unwrap();
            writer = z.finish().unwrap();
            let text = String::from_utf8(writer.clone()).expect("String parsing error");
            let decompressed: Vec<&str> = text.split("&").collect();
            let mut buffer = Vec::new();
            ureq::get(&format!("https://cdn.discordapp.com/attachments/{}/{}/data", decompressed.get(0).unwrap(), decompressed.get(1).unwrap())).call().unwrap().into_reader().read_to_end(&mut buffer).unwrap();
            let mut writer = Vec::new();
            let mut z = ZlibDecoder::new(writer);
            z.write_all(&buffer).unwrap();
            writer = z.finish().unwrap();
            let text = String::from_utf8(writer.clone()).expect("String parsing error");
            let mut decompressed: Vec<&str> = text.split("&").collect();
            let file_name = decode(decompressed[0]).unwrap();
            decompressed.remove(0);
            let channel = decompressed[0];
            decompressed.remove(0);
            stream.write(format!("HTTP/1.1 200 Ok\r\nContent-Disposition: attachment; filename=\"{}\"\r\n\r\n", file_name).as_bytes()).unwrap();
            let mut index = 0;
            for id in decompressed {
                let mut buffer = Vec::new();
                ureq::get(&format!("https://cdn.discordapp.com/attachments/{}/{}/part_{}", channel, id, index)).call().unwrap().into_reader().read_to_end(&mut buffer).unwrap();
                stream.write(&buffer).unwrap();
                index += 1;
            }
            stream.flush().unwrap();
        });
    }
}
