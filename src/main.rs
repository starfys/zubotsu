use serenity::client::Client;
use serenity::framework::Framework;
use serenity::model::channel::Message;
use serenity::model::id::EmojiId;
use serenity::model::misc::EmojiIdentifier;
use serenity::prelude::{Context, EventHandler};
use threadpool::ThreadPool;

use std::env;

mod data;

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    // Set the client to use our dank rust framework
    client.with_framework(ZubotsuFramework::new());

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct Handler;

impl EventHandler for Handler {}

struct ZubotsuFramework {
    stallman: bool,
}

impl ZubotsuFramework {
    fn new() -> Self {
        ZubotsuFramework { stallman: true }
    }
}

impl Framework for ZubotsuFramework {
    fn dispatch(&mut self, _: Context, message: Message, _: &ThreadPool) {
        // Convert the message to lowercase for string matching
        let message_text = message.content.to_lowercase();
        // check if someone's talking about DANK PROGRAMMING LANGUAGES
        if message_text.contains("rust") {
            // Construct the rust emoji
            let rust_emoji = EmojiIdentifier {
                id: EmojiId(539907481095110676),
                name: "rust".to_string(),
            };
            // Respond with the rust emoji
            let _ = message.react(rust_emoji);
        }
        // emulate Kinser
        if message_text.contains("map") {
            let _ = message.react("🗺");
        }
        // Stallman
        if self.stallman {
            if message_text == "stop stallman" {
                self.stallman = false;
                let _ = message.reply("Okay, but just know that Stallman is watching");
            } else if message_text.contains("linux") && !message_text.contains("gnu") {
                let _ = message.reply(data::GNU_LINUX_COPYPASTA);
            }
        } else {
            if message_text == "start stallman" {
                self.stallman = true;
                let _ = message.reply("*cracks knuckles* it's Free Software time");
            }
        }
    }
}
