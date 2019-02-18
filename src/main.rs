use serenity::client::Client;
use serenity::framework::Framework;
use serenity::model::channel::Message;
use serenity::model::id::EmojiId;
use serenity::model::misc::EmojiIdentifier;
use serenity::prelude::{Context, EventHandler};
use threadpool::ThreadPool;

use std::env;

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    // Set the client to use our dank rust framework
    client.with_framework(MyFramework);

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct Handler;

impl EventHandler for Handler {}

struct MyFramework;

impl Framework for MyFramework {
    fn dispatch(&mut self, _: Context, message: Message, _: &ThreadPool) {
        // Convert the message to lowercase for string matching
        let message_text = message.content.to_lowercase();
        // check if someone's talking about DANK PROGRAMMING LANGUAGES
        if message_text.contains("rust") {
            let rust_emoji = EmojiIdentifier {
                id: EmojiId(539907481095110676),
                name: "rust".to_string(),
            };
            let _ = message.react(rust_emoji);
        }
        // emulate Kinser
        if message_text.contains("map") {
            let _ = message.react("🗺");
        }
    }
}
