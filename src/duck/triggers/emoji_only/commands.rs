// emoji_only/commands.rs
use super::super::super::datatypes::{
    DuckMessages, EmojiModeEntity, EmojiModeLog, EmojiModeStates,
};
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;

/// Sets the emoji mode state of the first mentioned user (or channel, if no user
/// was pinged) to the value of `state`.  If `state` is not provided, emoji mode
/// is toggled for that entity.
fn set_emoji_mode(ctx: &mut Context, msg: &Message, state: Option<bool>) -> CommandResult {
    let emoji_entity = match msg.mentions.len() {
        0 => {
            // No user mentioned, this is just a channel
            EmojiModeEntity::Channel(msg.channel_id)
        }
        _ => EmojiModeEntity::User(msg.mentions[0].id),
    };

    let (current_emoji_state, new_emoji_state) = {
        let data = ctx.data.read();
        let emoji_states = data
            .get::<EmojiModeStates>()
            .expect("Expected EmojiModeStates in ShareMap");

        (
            emoji_states.contains(&emoji_entity),
            match state {
                Some(state) => state,
                None => !emoji_states.contains(&emoji_entity),
            },
        )
    };

    if new_emoji_state != current_emoji_state {
        {
            let mut data = ctx.data.write();

            let emoji_states = data
                .get_mut::<EmojiModeStates>()
                .expect("Expected EmojiModeStates in ShareMap");

            if !emoji_states.contains(&emoji_entity) {
                emoji_states.insert(emoji_entity);
            } else {
                emoji_states.remove(&emoji_entity);
            }
        }

        let emoji_message_start = match &emoji_entity {
            EmojiModeEntity::Channel(_) => String::from("This channel is"),
            EmojiModeEntity::User(user_id) => MessageBuilder::new()
                .mention(user_id)
                .push(", you are")
                .build(),
        };

        let data = ctx.data.read();
        let duck_messages = data
            .get::<DuckMessages>()
            .expect("Expected DuckMessages in ShareMap");
        let emoji_message_end = match new_emoji_state {
            true => duck_messages
                .get("emoji_mode_on_notification_end")
                .expect("Expected 'emoji_mode_on_notification_end' in DuckMessages"),
            false => duck_messages
                .get("emoji_mode_off_notification_end")
                .expect("Expected 'emoji_mode_on_notification_end' in DuckMessages"),
        };

        let emoji_message = format!("{} {}", emoji_message_start, emoji_message_end);

        if let Err(why) = msg.channel_id.say(&ctx, emoji_message) {
            return Err(CommandError(format!("{:?}", why)));
        }
    }

    Ok(())
}

#[command]
#[description = "Toggles only allowing emojis in messages"]
pub fn emoji(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(why) = match args.current() {
        Some("on") => set_emoji_mode(ctx, msg, Some(true)),
        Some("off") => set_emoji_mode(ctx, msg, Some(false)),
        _ => set_emoji_mode(ctx, msg, None),
    } {
        return Err(CommandError(format!("{:?}", why)));
    }

    let data = ctx.data.read();
    let emoji_states = data
        .get::<EmojiModeStates>()
        .expect("Expected EmojiModeStates in ShareMap");

    let users: HashSet<UserId> = emoji_states
        .iter()
        .filter(|entity| match entity {
            EmojiModeEntity::User(_) => true,
            _ => false,
        })
        .map(|entity| match entity {
            EmojiModeEntity::User(user) => user.clone(),
            _ => panic!("Something went horribly wrong when generating the user hashset!"),
        })
        .collect();

    let channels: HashSet<ChannelId> = emoji_states
        .iter()
        .filter(|entity| match entity {
            EmojiModeEntity::Channel(_) => true,
            _ => false,
        })
        .map(|entity| match entity {
            EmojiModeEntity::Channel(channel) => channel.clone(),
            _ => panic!("Something went horribly wrong when generating the user hashset!"),
        })
        .collect();

    let emoji_struct = EmojiModeLog { users, channels };

    let emoji_mode_file = File::create("database/emoji_mode.json")?;
    let emoji_mode_writer = BufWriter::new(emoji_mode_file);

    if let Err(why) = serde_json::to_writer_pretty(emoji_mode_writer, &emoji_struct) {
        return Err(CommandError(format!("{:?}", why)));
    }

    Ok(())
}
