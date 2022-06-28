use std::fs::File;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use std::process;

use clap::{App, Arg};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use lazy_static::lazy_static;
use pbr::ProgressBar;
use serenity::async_trait;
use serenity::Client;
use serenity::client::EventHandler;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Ready;
use serenity::prelude::Context;

lazy_static! {
    pub static ref MATCHES: clap::parser::ArgMatches = App::new("Sharex")
        .version("1.0")
        .about("Share big files on discord")
        .arg(
            Arg::with_name("Token")
                .short('t')
                .long("token")
                .help("The discord bot token")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("Application Id")
                .short('a')
                .long("application")
                .help("The discord bot application id")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("Channel")
                .short('c')
                .long("channel")
                .help("The channel id that the files will be uploaded to")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("Input")
                .short('i')
                .long("input")
                .help("The file to upload")
                .takes_value(true)
                .required(true)
        )
        .get_matches();
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(MATCHES.value_of("Token").unwrap())
        .event_handler(Handler)
        .application_id(MATCHES.value_of("Application Id").unwrap().parse().unwrap())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let channel = ChannelId(
            MATCHES
                .value_of("Channel")
                .unwrap()
                .parse()
                .expect("Channel needs to be a number"),
        );
        let mut file = File::open(MATCHES.value_of("Input").unwrap()).expect("no file found");
        if file.metadata().unwrap().len() < 8388608 {
            println!("The file is less than 8mb you can just upload it your self");
        } else {
            let total = (file.metadata().unwrap().len() as f64 / 8388608.0).ceil() as usize;
            let mut pb = ProgressBar::new((total * 2) as u64);
            pb.format("╢▌▌░╟");
            let mut i = 0;
            let mut returns = format!(
                "{}&{}",
                urlencoding::encode(
                    PathBuf::from(MATCHES.value_of("Input").unwrap())
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                ),
                MATCHES.value_of("Channel").unwrap()
            );
            while total - i > 0 {
                let mut buf = Vec::new();
                let mut temp = file.try_clone().expect("cant clone").take(8388608);
                file.seek(SeekFrom::Start((8388608 * i) as u64))
                    .expect("Failed to seek");
                temp.read_to_end(&mut buf).expect("Didn't read enough");
                pb.inc();
                let mut message = None;
                let mut attachments = None;
                let mut go_again = true;
                while go_again {
                    message = Some(
                        channel
                            .send_message(&ctx.http, |m| {
                                m.add_file((buf.as_slice(), format!("part_{}", i).as_str()));
                                m
                            })
                            .await,
                    );
                    go_again = message.as_ref().unwrap().is_err();
                    if !go_again {
                        attachments = Some(message.unwrap().unwrap().attachments);
                    }
                }
                pb.inc();
                for i in attachments.unwrap() {
                    returns.push_str(format!("&{}", &i.id.0.to_string()).as_str());
                }
                i += 1;
            }
            let base = "https://amtitan-sharex.herokuapp.com/";
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(returns.as_bytes()).unwrap();
            let data = e.finish().unwrap();
            let mut message = None;
            let mut attachments = None;
            let mut go_again = true;
            while go_again {
                message = Some(
                    channel
                        .send_message(&ctx.http, |m| {
                            m.add_file((data.as_slice(), "data"));
                            m
                        })
                        .await,
                );
                go_again = message.as_ref().unwrap().is_err();
                if !go_again {
                    attachments = Some(message.unwrap().unwrap().attachments);
                }
            }
            let char_list: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
            let mut returns = "".to_string();
            returns.push(*char_list.get(channel.0.to_string().len()).unwrap());
            for num in [channel.0, attachments.unwrap()[0].id.0] {
                let mut index = 0;
                loop {
                    let x = num/(62_u64.pow(index))%62;
                    if x == 0 {
                        break;
                    }
                    returns.push(*char_list.get(x as usize).unwrap());
                    index+=1;
                    if index == 11 {
                        break;
                    }
                }
            }
            pb.finish_println(format!("{}{}\n", base, returns).as_str());
        }
        process::exit(1);
    }
}
