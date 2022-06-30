use std::{process, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Duration;

use clap::{App, Arg};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use pbr::ProgressBar;
use rand::prelude::IteratorRandom;
use serde_json::Value;

fn main() {
    let matches: clap::parser::ArgMatches = App::new("Sharex")
        .version("1.0")
        .about("Share big files on discord")
        .arg(
            Arg::with_name("Webhook")
                .short('w')
                .long("webhook")
                .help("the url to the webhook")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("Input")
                .short('i')
                .long("input")
                .help("The file to upload")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let mut file = File::open(matches.value_of("Input").unwrap()).expect("no file found");
    if file.metadata().unwrap().len() < 8388608 {
        println!("The file is less than 8mb you can just upload it your self");
    } else {
        let total = (file.metadata().unwrap().len() as f64 / 8388608.0).ceil() as usize;
        let mut pb = ProgressBar::new((total * 2) as u64);
        pb.format("╢▌▌░╟");
        let mut i = 0;
        let channel: Value = serde_json::from_str(
            ureq::get(matches.value_of("Webhook").unwrap())
                .call()
                .unwrap()
                .into_string()
                .unwrap()
                .as_str(),
        )
            .unwrap();
        let channel = channel["channel_id"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let mut returns = format!(
            "{}&{}",
            urlencoding::encode(
                PathBuf::from(matches.value_of("Input").unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ),
            file.metadata().unwrap().len()
        );
        let mut gone = 0;
        while file.metadata().unwrap().len() > gone {
            let mut boundary = "--------".to_string();
            let mut rng = rand::thread_rng();
            for _ in 0..20 {
                boundary.push("0123456789".chars().choose(&mut rng).unwrap());
            }
            let mut buf = Vec::new();
            for i in format!("--{}\r\nContent-Disposition: form-data; name=\"file1\"; filename=\"{}\"\r\nContent-Type: application/octet-stream\r\n\r\n", boundary, format!("part_{}", i)).as_bytes() {
                buf.push(*i);
            }
            let offset = buf.len() + 34;
            let mut temp = file
                .try_clone()
                .expect("cant clone")
                .take((8388608 - offset) as u64);
            file.seek(SeekFrom::Start(gone)).expect("Failed to seek");
            gone += (8388608 - offset) as u64;
            temp.read_to_end(&mut buf).unwrap();
            for i in format!("\r\n--{}--", boundary).as_bytes() {
                buf.push(*i);
            }
            pb.inc();
            let mut response: Option<Value> = None;
            loop {
                let x = ureq::post(matches.value_of("Webhook").unwrap())
                    .set(
                        "content-type",
                        &format!("multipart/form-data; boundary={}", boundary),
                    )
                    .send_bytes(buf.as_slice());
                if x.is_ok() {
                    response = Some(
                        serde_json::from_str(x.unwrap().into_string().unwrap().as_str()).unwrap(),
                    );
                    break;
                } else {
                    println!("{:?}", x);
                    thread::sleep(Duration::from_secs(1));
                };
            }
            pb.inc();
            for i in response.unwrap()["attachments"].as_array().unwrap() {
                returns.push_str(format!("&{}", i["id"].as_str().unwrap()).as_str());
            }
            i += 1;
        }
        let base = "https://amtitan-sharex.herokuapp.com/";
        let mut boundary = "--------".to_string();
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            boundary.push("0123456789".chars().choose(&mut rng).unwrap());
        }
        let mut buf = Vec::new();
        for i in format!("--{}\r\nContent-Disposition: form-data; name=\"file1\"; filename=\"data\"\r\nContent-Type: application/octet-stream\r\n\r\n", boundary).as_bytes() {
            buf.push(*i);
        }
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(returns.as_bytes()).unwrap();
        for i in e.finish().unwrap() {
            buf.push(i);
        }
        for i in format!("\r\n--{}--", boundary).bytes() {
            buf.push(i);
        }
        let mut response: Option<Value> = None;
        loop {
            let x = ureq::post(matches.value_of("Webhook").unwrap())
                .set(
                    "content-type",
                    &format!("multipart/form-data; boundary={}", boundary),
                )
                .send_bytes(buf.as_slice());
            if x.is_ok() {
                response =
                    Some(serde_json::from_str(x.unwrap().into_string().unwrap().as_str()).unwrap());
                break;
            } else {
                println!("{:?}", x);
                thread::sleep(Duration::from_secs(1));
            };
        }
        let char_list: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            .chars()
            .collect();
        let mut returns = "".to_string();
        returns.push(*char_list.get(channel.to_string().len()).unwrap());
        for num in [
            channel,
            response.unwrap()["attachments"][0]["id"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        ] {
            let mut index = 0;
            loop {
                let x = num / (62_u64.pow(index)) % 62;
                returns.push(*char_list.get(x as usize).unwrap());
                index += 1;
                if index == 11 {
                    break;
                }
            }
        }
        pb.finish_println(format!("{}{}\n", base, returns).as_str());
    }
    process::exit(1);
}
