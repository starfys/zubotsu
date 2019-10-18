use serenity::model::id::EmojiId;

use std::collections::HashMap;

pub const RUST: EmojiId = EmojiId(539907481095110676);
pub const HASKELL: EmojiId = EmojiId(540376527674540048);

pub const MEGADOWNBISHOY00: EmojiId = EmojiId(544751298096922626);
pub const MEGADOWNBISHOY01: EmojiId = EmojiId(544751281978474496);
pub const MEGADOWNBISHOY10: EmojiId = EmojiId(544751272524251136);
pub const MEGADOWNBISHOY11: EmojiId = EmojiId(544751261443031061);

/// Emojis

/// Takes a string, and generates a sequence of emojis that spells out that string
///
/// # Parameters
/// * `message` -- The message to emojify
///
/// # Returns
/// A Vec of emojis
pub fn emojify(message: &str) -> Vec<&str> {
    // TODO: make this lazy static (inside the function)
    // Construct emoji map
    let mut emoji_map: HashMap<char, Vec<&'static str>> = HashMap::new();
    emoji_map.insert('a', vec!["🇦", "🅰"]);
    emoji_map.insert('b', vec!["🇧", "🅱"]);
    emoji_map.insert('c', vec!["🇨", "©", "🌜"]);
    emoji_map.insert('d', vec!["🇩"]);
    emoji_map.insert('e', vec!["🇪", "📧"]);
    emoji_map.insert('f', vec!["🇫", "🎏"]);
    emoji_map.insert('g', vec!["🇬"]);
    emoji_map.insert('h', vec!["🇭", "♓"]);
    emoji_map.insert('i', vec!["🇮", "ℹ", "🌵", "🚦", "🛢", "🕯", "📍", "🎚"]);
    emoji_map.insert('j', vec!["🇯", "🗾", "🏒"]);
    emoji_map.insert('k', vec!["🇰", "🎋"]);
    emoji_map.insert('l', vec!["🇱"]);
    emoji_map.insert('m', vec!["🇲", "Ⓜ", "〽️", "Ⓜ️", "♍️"]); // TODO: apparently discord doesn't like all of these
    emoji_map.insert('n', vec!["🇳"]);
    emoji_map.insert('o', vec!["🇴", "🅾", "🅾️", "🌕", "🌚", "🌝", "⚙"]);
    emoji_map.insert('p', vec!["🇵", "🅿", "🅿️"]);
    emoji_map.insert('q', vec!["🇶"]);
    emoji_map.insert('r', vec!["🇷", "®"]);
    emoji_map.insert('s', vec!["🇸", "⚡"]);
    emoji_map.insert('t', vec!["🇹", "✝", "☦", "🌴", "⛏"]);
    emoji_map.insert('u', vec!["🇺", "⛎"]);
    emoji_map.insert('v', vec!["🇻", "♈", "✅", "✔️", "☑️"]);
    emoji_map.insert('w', vec!["🇼"]);
    emoji_map.insert('x', vec!["🇽", "⚔", "❌", "❎"]);
    emoji_map.insert('y', vec!["🇾"]);
    emoji_map.insert('z', vec!["🇿"]);

    emoji_map.insert('!', vec!["‼", "❗️", "❕"]);
    emoji_map.insert('?', vec!["❓", "❔"]);

    emoji_map.insert('9', vec!["9⃣"]);
    emoji_map.insert('7', vec!["7⃣"]);
    emoji_map.insert('8', vec!["8⃣"]);
    emoji_map.insert('6', vec!["6⃣"]);
    emoji_map.insert('5', vec!["5⃣"]);
    emoji_map.insert('4', vec!["4⃣"]);
    emoji_map.insert('3', vec!["3⃣"]);
    emoji_map.insert('2', vec!["2⃣"]);
    emoji_map.insert('1', vec!["1⃣"]);
    emoji_map.insert('0', vec!["0⃣"]);
    

    // TODO: may make it easy to just make this a hashset
    let mut visited_chars: HashMap<char, usize> = HashMap::new();
    
    let mut emojis = Vec::new();
    for character in message.chars() {
        if let Some(emoji_list) = emoji_map.get(&character) {
            //TODO: convert this to entry API
            // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
            let emoji_index = match visited_chars.get(&character) {
                Some(index) => *index as usize,
                None => 0 as usize,
            };
            visited_chars.insert(character, emoji_index + 1);
            if emoji_index < emoji_list.len() {
                emojis.push(emoji_list[emoji_index]);
            }
        };
    }
    emojis
}
