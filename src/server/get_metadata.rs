use std::io::{Read, Write};

use flate2::write::ZlibDecoder;
use urlencoding::decode;

pub fn get_metadata(returns: [u64; 2]) -> Option<(String, u64, Vec<String>)> {
    let mut buffer = Vec::new();
    ureq::get(&format!(
        "https://cdn.discordapp.com/attachments/{}/{}/data",
        returns[0], returns[1]
    ))
        .call()
        .ok()?
        .into_reader()
        .read_to_end(&mut buffer)
        .ok()?;
    let mut writer = Vec::new();
    let mut z = ZlibDecoder::new(writer);
    z.write_all(&buffer).ok()?;
    writer = z.finish().ok()?;
    let text = String::from_utf8(writer.clone()).ok()?;
    let mut decompressed: Vec<String> = text.split("&").map(|s| s.to_string()).collect();
    let decompressed_cloned = decompressed[0].clone();
    let file_name = decode(&decompressed_cloned).ok()?;
    decompressed.remove(0);
    let length = decompressed[0].parse::<u64>().ok()?;
    decompressed.remove(0);
    Some((file_name.to_string(), length, decompressed))
}
