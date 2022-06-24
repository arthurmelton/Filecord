use lazy_static::lazy_static;
use serenity::Client;
use serenity::client::EventHandler;
use serenity::model::prelude::Ready;
use serenity::prelude::Context;

lazy_static! {
    pub static ref matches = App::new("Sharex")
        .version("1.0")
        .about("Share big files on discord")
        .arg(Arg::with_name("Token")
            .short("t")
            .long("token")
            .help("The discord bot token")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("Channel")
            .short("c")
            .long("channel")
            .help("The channel id that the files will be uploaded to")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("Input")
            .short("i")
            .long("input")
            .help("The file to upload")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("Output")
            .short("o")
            .long("output")
            .help("The location for the txt file")
            .takes_value(true)
            .required(true))
        .get_matches();
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(matches.value_of("Token").unwrap())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        crate::ready::ready::ready(ctx, ready).await;
    }
}
