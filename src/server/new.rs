use std::io::Write;
use crate::server::read::read;
use crate::server::error::error;
use crate::server::messaging_platform::messaging_platform;

use std::net::TcpStream;
use crate::server::get_metadata::get_metadata;
use crate::server::get_path::get_path;
use crate::server::send_messaging_platform::send_messaging_platform;
use crate::server::send_page::send_page;
use crate::server::send_to_user::send_to_user;
use crate::utils::asset::Asset;
use crate::utils::decrypt_path::decrypt_path;

pub fn new(mut stream: TcpStream) {
    let response = read(&stream);
    match get_path(response.clone()) {
        Some(path) => {
            if path == "" || Asset::get(&path).is_some() {
                send_page(stream.try_clone().unwrap(), path);
            }
            else {
                match decrypt_path(path) {
                    Some(ids) => {
                        let channel = ids[0];
                        match get_metadata(ids) {
                            Some((filename, length, ids)) => {
                                if messaging_platform(response) {
                                    send_messaging_platform(stream.try_clone().unwrap(), filename);
                                }
                                else {
                                    send_to_user(filename, length, ids, channel, stream.try_clone().unwrap());
                                }
                            },
                            None => error(stream.try_clone().unwrap(), 404, "Not Found", response)
                        }
                    },
                    None => error(stream.try_clone().unwrap(), 404, "Bad Request", response)
                }
            }
        },
        None => error(stream.try_clone().unwrap(), 400, "Bad Request", response)
    }
    stream.flush().unwrap();
}