#![feature(custom_attribute)]
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

mod data;
mod db;
mod emoji;
pub mod models;
pub mod schema;

use chrono::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use log::{debug, error};
use meval;
use serenity::client::Client;
use serenity::framework::Framework;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::misc::EmojiIdentifier;
use serenity::prelude::{Context, EventHandler};
use std::env;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    pretty_env_logger::init();

    // Import environment variables from .env
    dotenv().ok();
    // Get the database URL
    let database_url = env::var("DATABASE_URL")?;
    // Get the bot token
    let bot_token = env::var("DISCORD_TOKEN")?;
    // Login with a bot token from the environment
    let mut client = Client::new(&bot_token, Handler)?;

    // Initialize the framework
    let framework = ZubotsuFramework::new(&database_url)?;

    // Set the client to use our dank rust framework
    client.with_framework(framework);

    // Start the client
    client.start_autosharded()?;

    Ok(())
}

struct Handler;

impl EventHandler for Handler {}

struct ZubotsuFramework {
    free_software: Arc<AtomicBool>,
    db_conn: Arc<Mutex<PgConnection>>,
}

impl ZubotsuFramework {
    fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        let conn = db::establish_connection(database_url)?;

        Ok(ZubotsuFramework {
            free_software: Arc::new(AtomicBool::new(false)),
            db_conn: Arc::new(Mutex::new(conn)),
        })
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
            } else if message_text == "start free software" || message_text == "start miku" {
                free_software.store(true, Ordering::SeqCst);
                let _ = message.reply(&context, "*cracks knuckles* it's Miku time");
            }
            // the one true time
            if message_text.contains("time in beats") {
                let minute = 60;
                let hour = 60 * minute;

                let internet_timezone = FixedOffset::east(hour as i32);

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
                    if let Err(e) = message.reply(&context, "Nothing to respond with") {
                        error!("error while reacting {}", e);
                    }
                } else {
                    // by default use a reaction if the number of reactable emojis are lower than 20 ( discord emoji limit)
                    // by default use a reply if above
                    // add a new command to force a reply message
                    // add a new command to force a react message
                    // if there are no availible emojis force a reply message
                    let emoji_map = emoji::emojify(message_text.trim_start_matches("react "));
                    if message_text.starts_with("reply ")
                        || ((emoji_map.len() > emoji::MAX_REACTION_LIMIT || emoji_map.is_empty())
                            && !message_text.starts_with("react "))
                    {
                        let message_text = message_text.trim_start_matches("reply ");
                        if let Err(err) = message
                            .channel_id
                            .say(&context, emoji::emoji_replace(message_text))
                        {
                            error!("error while replying {}", err);
                        }
                    } else {
                        for (index, emoji) in emoji_map.iter().enumerate() {
                            if index == emoji::MAX_REACTION_LIMIT {
                                error!(
                                    "message_text {} too long, cutting it off now",
                                    message_text
                                );
                                break;
                            }
                            if let Err(err) = message.react(&context, *emoji) {
                                error!("error while reacting {} {:?}", err, *emoji);
                            }
                        }
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

                let _ = message.channel_id.say(
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
                let guild_id = message.guild_id;
                // let's get access to the db conn
                let locked_conn = conn.lock().unwrap();
                let command = message_text.split(' ').collect::<Vec<&str>>();
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
                                    let user = match UserId::to_user(UserId(user_id), &context) {
                                        Err(e) => {
                                            error!("unknown id {} {}", user_id, e);
                                            format!("unknown id {}", user_id)
                                        }
                                        Ok(user) => match guild_id {
                                            Some(guild_id) => {
                                                match user.nick_in(&context, guild_id) {
                                                    Some(nick_name) => nick_name,
                                                    None => user.name,
                                                }
                                            }
                                            None => user.name,
                                        },
                                    };
                                    let karma_amount = match karma_user.karma {
                                        Some(karma_amount) => karma_amount,
                                        None => 0,
                                    };
                                    format!("{}. {} : {}", index + 1, user, karma_amount)
                                });
                                // TODO: update this so that we collect using `.map(|s| &**s)`
                                // instead so we can borrow these strings
                                if let Err(e) = message.channel_id.say(
                                    &context,
                                    format!(
                                        "High Scores \n{}",
                                        format.collect::<Vec<String>>().join("\n")
                                    ),
                                ) {
                                    error!("{}", e);
                                }
                            }
                        }
                    // TODO: do we want to only have the ability to look at your own karma, or anyone's on the server
                    } else if message.mentions.len() == 1 {
                        let result = db::get_karma_for_id(&*locked_conn, message.mentions[0].id.0);
                        match result {
                            Ok(karma) => {
                                if let Err(err) =
                                    message.reply(&context, format!("Here's their karma {}", karma))
                                {
                                    error!("reply error {}", err);
                                }
                            }
                            Err(err) => {
                                error!("{}", err);
                                if let Err(err) =
                                    message.reply(&context, "Could not retrieve data for user {}")
                                {
                                    error!("reply error {}", err);
                                }
                            }
                        }
                    }
                // karmabot 69 @(apply #(lube %) (your butt))
                } else if command.len() > 2 {
                    let eval_expr = message_text
                        .trim_start_matches("karmabot ")
                        .replace(' ', "");
                    let eval_expr = eval_expr.split("<@").next();
                    match eval_expr {
                        Some("") | None => {
                            if let Err(err) = message.reply(&context, format!("empty command")) {
                                error!("reply error: {}", err);
                            }
                        }
                        Some(eval_expr) => {
                            let karma_amount = match meval::eval_str(eval_expr) {
                                Err(err) => {
                                    error!("reply error: {}", err);
                                    0
                                }
                                Ok(value) => value as i32,
                            };
                            if karma_amount == 0 {
                                if let Err(err) = message
                                    .reply(&context, format!("invalid command {}", eval_expr))
                                {
                                    error!("reply error: {}", err);
                                };
                            } else {
                                for mention in message.mentions {
                                    match db::upsert_user_karma(
                                        &*locked_conn,
                                        mention.id.0,
                                        karma_amount,
                                    ) {
                                        Err(err) => error!("upsert db error: {}", err),
                                        _ => debug!(
                                            "added {} karma for {}",
                                            karma_amount, mention.id.0
                                        ),
                                    };
                                }
                            }
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
