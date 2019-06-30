// emoji_only/commands.rs
use super::super::super::datatypes::{
    DuckIdentity, DuckMessages, EmojiList, EmojiModeEntity, EmojiModeStates, Identity,
};
use super::super::super::utils;
use regex::Regex;
use serenity::framework::standard::{macros::check, Args, CheckResult, CommandOptions};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn valid_emoji_message(emoji_set: &HashSet<String>, msg: &str) -> bool {
    let non_emoji_str: String = UnicodeSegmentation::graphemes(msg, true)
        .filter(|grapheme| !emoji_set.contains(&String::from(*grapheme)))
        .collect();

    let re = Regex::new("<:[a-zA-Z]+:(\\d+)>").unwrap();
    for emoji_id in re.captures_iter(&non_emoji_str) {
        let url = format!("https://cdn.discordapp.com/emojis/{}", &emoji_id[1]);
        match &reqwest::get(&url) {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }

    re.replace_all(&non_emoji_str, "").is_empty()
}

#[check]
#[name = "EmojiMode"]
#[display_in_help(false)]
//#[check_in_help(true)]
pub fn message_check(
    ctx: &mut Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    allowed_message(ctx, msg).into()
}

pub fn allowed_message(ctx: &mut Context, msg: &Message) -> bool {
    if msg.author.bot
        || match &msg.member {
            Some(member) => utils::is_admin(&ctx, &member),
            None => false,
        }
    {
        return true;
    }

    let data = ctx.data.read();
    let emoji_entities = data
        .get::<EmojiModeStates>()
        .expect("Expected EmojiModeStates in ShareMap");

    let emoji_only_channel = emoji_entities.contains(&EmojiModeEntity::Channel(msg.channel_id));
    let emoji_only_user = emoji_entities.contains(&EmojiModeEntity::User(msg.author.id));

    if !emoji_only_channel && !emoji_only_user {
        return true;
    }

    let emoji_set = data
        .get::<EmojiList>()
        .expect("Expected EmojiList in ShareMap");

    let curr_duck = data
        .get::<DuckIdentity>()
        .expect("Expected DuckIdentity in ShareMap");

    if !valid_emoji_message(emoji_set, &msg.content) && curr_duck == &Identity::RubberDuck {
        if let Err(why) = msg.delete(&ctx) {
            eprintln!("Error removing emoji message: {:?}", why);
        }
        send_warning(ctx, &msg.author);
        return false;
    }

    true
}

fn send_warning(ctx: &Context, offender: &User) {
    let data = ctx.data.read();
    let messages = data
        .get::<DuckMessages>()
        .expect("Expected DuckMessages in ShareMap");
    let warning_message = messages
        .get("emoji_mode_violation_message")
        .expect("Expected 'emoji_only_violation_message' in DuckMessages");

    if let Err(why) = offender.direct_message(ctx, |m| m.content(warning_message)) {
        eprintln!("Error sending emoji-only violation warning: {:?}", why);
    }
}
