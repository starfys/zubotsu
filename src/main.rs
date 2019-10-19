#![feature(custom_attribute, proc_macro)]
// Copyright 2019 Steven Sheffey
// This file is part of Zubotsu.

// Zubotsu is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Zubotsu is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Zubotsu.  If not, see <https://www.gnu.org/licenses/>.
#[macro_use]
extern crate diesel;

use chrono::prelude::*;
use dotenv::dotenv;
use log::{debug, error, info};
use serenity::client::Client;
use serenity::framework::Framework;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::misc::EmojiIdentifier;
use serenity::prelude::{Context, EventHandler};
use threadpool::ThreadPool;
use std::env;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,Mutex};
use diesel::pg::PgConnection;

mod emoji;
mod data;
mod db;

pub mod models;
pub mod schema;

fn main() {
    dotenv::ok();
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
    // dao: &'T db::DAO<'T>,
    db_conn: Arc<Mutex<PgConnection>>,
}

impl ZubotsuFramework {
    fn new() -> Self {
        let conn = db::establish_connection().expect("could not establish connection");

        ZubotsuFramework {
            free_software: Arc::new(AtomicBool::new(false)),
            db_conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl Framework for ZubotsuFramework {
    fn dispatch(&mut self, context: Context, message: Message, threadpool: &ThreadPool) {
        // Clone a message reference
        let free_software = self.free_software.clone();
        let conn = self.db_conn.clone();
        // Handle the message in another thread
        threadpool.execute(move || {
            // Convert the message to lowercase for string matching
            let message_text = message.content.to_lowercase();
            // check if someone's talking about DANK PROGRAMMING LANGUAGES
            if message_text.contains("rust") {
                // Construct the rust emoji
                let rust_emoji = EmojiIdentifier {
                    id: emoji::RUST,
                    name: "rust".to_string(),
                };
                // Respond with the rust emoji
                let _ = message.react(&context, rust_emoji);
            }
            // check to see if someone's talking about THE ULTIMATE LIFE FORM^H^H^H^H LANGUAGE
            if message_text.contains("haskell")
                || message_text.contains("monad")
                || message_text.contains("functor")
                || message_text.contains("typeclass")
            {
                // Construct the haskell emoji
                let haskell_emoji = EmojiIdentifier {
                    id: emoji::HASKELL,
                    name: "haskell".to_string(),
                };
                // Respond with the haskell emoji
                let _ = message.react(&context, haskell_emoji);
            }
            // emulate Kinser
            if message_text.contains("map") {
                let _ = message.react(&context, "🗺");
            }
            // Miku is the real leader of the gnu project
            if free_software.load(Ordering::SeqCst) {
                if message_text == "stop free software" || message_text == "stop miku" {
                    free_software.store(false, Ordering::SeqCst);
                    let _ = message.reply(&context, "Okay, but just know that miku is watching");
                } else if message_text.contains("linux") && !message_text.contains("gnu") {
                    let _ = message.reply(&context, data::GNU_LINUX_COPYPASTA);
                }
            } else {
                if message_text == "start free software" || message_text == "start miku" {
                    free_software.store(true, Ordering::SeqCst);
                    let _ = message.reply(&context, "*cracks knuckles* it's Miku time");
                }
            }
            // the one true time
            if message_text.contains("time in beats") {
                let minute = 60;
                let hour = 60 * minute;

                let internet_timezone = FixedOffset::east(1 * hour as i32);

                let maboi = Utc::now().with_timezone(&internet_timezone).time();
                let _ = message.reply(
                    &context,
                    &format!(
                        "The current Internet Time is @{:.3}.beats",
                        ((maboi.second() + maboi.minute() * minute + maboi.hour() * hour) as f64
                            + (maboi.nanosecond() as f64 / 1_000_000_000.0))
                            / 86.4
                    ),
                );
            }

            // the meme time
            if message_text.contains("time in scaramuccis") {
                let scaramucci_start = Utc.ymd(2017, 7, 25).and_hms(9, 0, 0).timestamp_millis();
                let scaramucci_end = Utc.ymd(2017, 7, 31).and_hms(9, 0, 0).timestamp_millis();
                let scaramucci_duration = scaramucci_end - scaramucci_start;
                let scaramucci_time = (Utc::now().timestamp_millis() - scaramucci_start) as f64
                    / (scaramucci_duration as f64);

                let _ = message.reply(
                    &context,
                    &format!(
                        "It has been {:.2} scaramuccis since the scaramucci muccied",
                        scaramucci_time
                    ),
                );
            }

            // emoji-ify the command
            if message_text.starts_with("zubotsu") {
                let message_text = message_text.trim_start_matches("zubotsu ");
                if message_text == "" {
                    let _ = message.reply(&context, "Nothing to respond with");
                } else {
                    for emoji in emoji::emojify(message_text) {
                        let _ = message.react(&context, emoji);
                    }
                }
            }
            if message_text == "!megadownbishoy" {
                let bishoy_emoji00 = EmojiIdentifier {
                    id: emoji::MEGADOWNBISHOY00,
                    name: "megadownbishoy00".to_string(),
                };

                let bishoy_emoji01 = EmojiIdentifier {
                    id: emoji::MEGADOWNBISHOY01,
                    name: "megadownbishoy01".to_string(),
                };

                let bishoy_emoji10 = EmojiIdentifier {
                    id: emoji::MEGADOWNBISHOY10,
                    name: "megadownbishoy10".to_string(),
                };

                let bishoy_emoji11 = EmojiIdentifier {
                    id: emoji::MEGADOWNBISHOY11,
                    name: "megadownbishoy11".to_string(),
                };

                let _ = message.reply(
                    &context,
                    format!(
                        "\n{}{}\n{}{}",
                        to_str(bishoy_emoji00).as_str(),
                        to_str(bishoy_emoji01).as_str(),
                        to_str(bishoy_emoji10).as_str(),
                        to_str(bishoy_emoji11).as_str()
                    )
                    .as_str(),
                );
            }

            // karmabot +69 @(apply #(lube %) (your butt))
            if message_text.starts_with("karmabot") {
                // let's get access to the db conn
                let locked_conn = conn.lock().unwrap();
                let command = message_text.split(" ").collect::<Vec<&str>>();
                // karmabot leaderboards
                // karmabot @(apply #(lube %) (your butt))
                if command.len() == 2 {
                    if command[1] == "leaderboards" {
                        match db::leaderboards(&*locked_conn) {
                            Err(err) => error!("{}", err),
                            Ok(users) => {
                                // do we want to move this formatting code out to a separate funtion
                                let format = users.iter().enumerate().map(|(index, karma_user)| {
                                    // this is technically unsafe transform but due to knowledge about the id system of discord
                                    // we can ignore this for now (until 2084)
                                    let user_id = karma_user.user_id as u64;
                                    let user =
                                        match UserId::to_user(UserId(user_id), &context) {
                                            Err(e) => {
                                                error!("unknown id {} {}", user_id, e);
                                                format!("unknown id {}", user_id)
                                            }
                                            Ok(user) => user.name,
                                        };
                                    let karma_amount = match karma_user.karma {
                                        Some(karma_amount) => karma_amount,
                                        None => 0,
                                    };
                                    format!("{}. {} : {}", index + 1, user.to_owned(), karma_amount)
                                });
                                // TODO: update this so that we collect using `.map(|s| &**s)`
                                // instead so we can borrow these strings
                                match message.reply(
                                    &context,
                                    format!(
                                        "High Scores \n{}",
                                        format.collect::<Vec<String>>().join("\n")
                                    ),
                                ) {
                                    Err(e) => error!("{}", e),
                                    Ok(_) => (),
                                }
                            }
                        }
                        // TODO: do we want to only have the ability to look at your own karma, or anyone's on the server
                    } else if message.mentions.len() == 1 {
                        let result = db::get_karma_for_id(&*locked_conn, message.mentions[0].id.0);
                        match result {
                            Err(err) => {
                                error!("{}", err);
                                match message.reply(&context, "Could not retrieve data for user {}") {
                                    Err(err) => error!("{}", err),
                                    Ok(_) => (),
                                }
                            }
                            Ok(karma) => match message
                                .reply(&context, format!("Here's their karma {}", karma))
                            {
                                Err(err) => error!("{}", err),
                                Ok(_) => (),
                            },
                        }
                    }
                // karmabot +69 @(apply #(lube %) (your butt))
                } else if command.len() > 2 {
                    let karma_amount = match command[1].replace("+", "").parse::<i32>() {
                        Err(err) => {
                            error!("{}", err);
                            0
                        }
                        Ok(value) => value,
                    };
                    if karma_amount == 0 {
                        match message.reply(&context, format!("invalid command {}", command[1])) {
                            Err(err) => error!("{}", err),
                            Ok(_) => (),
                        };
                    } else {
                        for mention in message.mentions {
                            match db::upsert_user_karma(&*locked_conn, mention.id.0, karma_amount) {
                                Err(err) => error!("{}", err),
                                _ => debug!("added {} karma for {}", karma_amount, mention.id.0),
                            };
                        }
                    }
                }
            }
        });
    }
}

// TODO: use some fancy traits for this or implicits?
fn to_str(emoji: EmojiIdentifier) -> String {
    format!("<:{}:{}>", emoji.name, emoji.id)
}
