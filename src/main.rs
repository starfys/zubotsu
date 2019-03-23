// Copyright 2019 Steven Sheffey
// This file is part of Zubotsu.

// Foobar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Foobar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Zubotsu.  If not, see <https://www.gnu.org/licenses/>.

use chrono::prelude::*;
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

    // start listening for events by starting one shard
    if let Err(why) = client.start_autosharded() {
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
    fn dispatch(&mut self, _context: Context, message: Message, threadpool: &ThreadPool) {
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
            let message = message.clone();
            threadpool.execute(move || {
                let _ = message.react(rust_emoji);
            });
        }
        // emulate Kinser
        if message_text.contains("map") {
            let _ = message.react("🗺");
        }
        // Stallman
        if self.stallman {
            if message_text == "stop stallman" {
                self.stallman = false;
                let message = message.clone();
                threadpool.execute(move || {
                    let _ = message.reply("Okay, but just know that Stallman is watching");
                });
            } else if message_text.contains("linux") && !message_text.contains("gnu") {
                let message = message.clone();
                threadpool.execute(move || {
                    let _ = message.reply(data::GNU_LINUX_COPYPASTA);
                });
            }
        } else {
            if message_text == "start stallman" {
                self.stallman = true;
                let message = message.clone();
                threadpool.execute(move || {
                    let _ = message.reply("*cracks knuckles* it's Free Software time");
                });
            }
        }
        // Megadownbishoy
        if message_text.starts_with("!megadownbishoy") {
            let message = message.clone();
            threadpool.execute(move || {
                let _ = message.reply(
                    ":megadownbishoy00::megadownbishoy01:\n:megadownbishoy10::megadownbishoy11:",
                );
            });
        }
        //the one true time
        if message_text.contains("time in beats") {
            let message = message.clone();
            let maboi = Utc::now().time();
            threadpool.execute(move || {
                let _ = message.reply(&format!(
                    "The current Internet Time is @{:.0}.beats",
                    (maboi.second() + (maboi.minute() * 60) + ((maboi.hour() + 1) * 3600)) as f32
                        / 86.4
                ));
            });
        }
    }
}
