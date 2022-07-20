use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "www"]
pub struct Asset;