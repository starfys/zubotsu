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
use std::collections::{HashMap, HashSet};
use threadpool::ThreadPool;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
    free_software: Arc<AtomicBool>,
    regional_indicator_map: HashMap<char, &'static str>,
}

impl ZubotsuFramework {
    fn new() -> Self {
        let mut regional_indicator_map = HashMap::new();
        regional_indicator_map.insert('a', "🇦");
        regional_indicator_map.insert('b', "🇧");
        regional_indicator_map.insert('c', "🇨");
        regional_indicator_map.insert('d', "🇩");
        regional_indicator_map.insert('e', "🇪");
        regional_indicator_map.insert('f', "🇫");
        regional_indicator_map.insert('g', "🇬");
        regional_indicator_map.insert('h', "🇭");
        regional_indicator_map.insert('i', "🇮");
        regional_indicator_map.insert('j', "🇯");
        regional_indicator_map.insert('k', "🇰");
        regional_indicator_map.insert('l', "🇱");
        regional_indicator_map.insert('m', "🇲");
        regional_indicator_map.insert('n', "🇳");
        regional_indicator_map.insert('o', "🇴");
        regional_indicator_map.insert('p', "🇵");
        regional_indicator_map.insert('q', "🇶");
        regional_indicator_map.insert('r', "🇷");
        regional_indicator_map.insert('s', "🇸");
        regional_indicator_map.insert('t', "🇹");
        regional_indicator_map.insert('u', "🇺");
        regional_indicator_map.insert('v', "🇻");
        regional_indicator_map.insert('w', "🇼");
        regional_indicator_map.insert('x', "🇽");
        regional_indicator_map.insert('y', "🇾");
        regional_indicator_map.insert('z', "🇿");
        ZubotsuFramework {
            free_software: Arc::new(AtomicBool::new(false)),
            regional_indicator_map: regional_indicator_map,
        }
    }
}

impl Framework for ZubotsuFramework {
    fn dispatch(&mut self, _context: Context, message: Message, threadpool: &ThreadPool) {
        // Clone a message reference
        let message = message.clone();
        let free_software = self.free_software.clone();
        let regional_indicator_map = self.regional_indicator_map.clone();
        // Handle the message in another thread
        threadpool.execute(move || {
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
                let _ = message.react(rust_emoji);
            }
            // check to see if someone's talking about THE ULTIMATE LIFE FORM^H^H^H^H LANGUAGE
            if message_text.contains("haskell")
                || message_text.contains("monad")
                || message_text.contains("functor")
                || message_text.contains("typeclass")
            {
                // Construct the haskell emoji
                let haskell_emoji = EmojiIdentifier {
                    id: EmojiId(540376527674540048),
                    name: "haskell".to_string(),
                };
                // Respond with the haskell emoji
                let message = message.clone();
                let _ = message.react(haskell_emoji);
            }
            // emulate Kinser
            if message_text.contains("map") {
                let _ = message.react("🗺");
            }
            // Miku is the real leader of the gnu project
            if free_software.load(Ordering::SeqCst) {
                if message_text == "stop free software" || message_text == "stop miku" {
                    free_software.store(false, Ordering::SeqCst);
                    let message = message.clone();
                    let _ = message.reply("Okay, but just know that free software is watching");
                } else if message_text.contains("linux") && !message_text.contains("gnu") {
                    let message = message.clone();
                    let _ = message.reply(data::GNU_LINUX_COPYPASTA);
                }
            } else {
                if message_text == "start free software" || message_text == "start miku" {
                    free_software.store(true, Ordering::SeqCst);
                    let message = message.clone();
                    let _ = message.reply("*cracks knuckles* it's Free Software time");
                }
            }
            // the one true time
            if message_text.contains("time in beats") {
                let message = message.clone();

                let minute = 60;
                let hour = 60 * minute;

                let internet_timezone = FixedOffset::east(1 * hour as i32);

                let maboi = Utc::now().with_timezone(&internet_timezone).time();
                let _ = message.reply(&format!(
                    "The current Internet Time is @{:.3}.beats",
                    ((maboi.second() + maboi.minute() * minute + maboi.hour() * hour) as f64
                        + (maboi.nanosecond() as f64 / 1_000_000_000.0))
                        / 86.4
                ));
            }

            // the meme time
            if message_text.contains("time in scaramuccis") {
                let message = message.clone();

                let scaramucci_start = Utc.ymd(2017, 7, 25).and_hms(9, 0, 0).timestamp_millis();
                let scaramucci_end = Utc.ymd(2017, 7, 31).and_hms(9, 0, 0).timestamp_millis();
                let scaramucci_duration = scaramucci_end - scaramucci_start;
                let scaramucci_time = (Utc::now().timestamp_millis() - scaramucci_start) as f64
                    / (scaramucci_duration as f64);

                let _ = message.reply(&format!(
                    "It has been {:.2} scaramuccis since the scaramucci muccied",
                    scaramucci_time
                ));
            }

            // emoji-ify the command 
            if message_text.starts_with("zubotsu") {
                let message = message.clone();
                let message_text = message_text.replacen("zubotsu","",1).replace(" ","");
                if message_text == "" {
                    let _ = message.reply("Nothing to respond with");
                } else {
                    if !message_text.chars().all(char::is_alphanumeric){
                        let _ = message.reply("Can only respond with alphanumerics");
                    } else {
                        // unless I want to handle duplicate emojis for a such as 🇦 and 🅰 then will have to make sure
                        // that each char only shows up once
                        let mut char_set = HashSet::new();
                        let mut is_nondistinct = false;
                        for character in message_text.chars() {
                            if !char_set.contains(&character) {
                                char_set.insert(character);
                            } else {
                                is_nondistinct = true;
                                break;
                            }
                        }
                        if is_nondistinct {
                            let _ = message.reply("Currently can only reply to words with only one emoji per character");
                        } else {
                            for character in message_text.chars() {
                                let _ = match regional_indicator_map.get(&character) {
                                    Some(emoji) => message.react(emoji.to_string()),
                                    None => std::result::Result::Ok(()),
                                };
                            }
                        }
                    }
                }
            }
        });
    }
}
