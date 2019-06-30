pub mod interduck;
pub mod robo_mallard;
pub mod rubber_duck;

use serenity::prelude::*;
use serenity::{
    framework::standard::StandardFramework,
    model::id::{ChannelId, GuildId},
};
use std::collections::HashSet;
use std::sync::{mpsc::Receiver, Arc, Mutex};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use super::datatypes::*;
use super::triggers;

pub fn both_ducks_init(
    client: &mut Client,
    which_duck: Identity,
) -> Result<(StandardFramework, Receiver<(ChannelId, String, usize)>), Box<Error>> {
    let duck_config = File::open("database/duck.json")?;
    let duck_reader = BufReader::new(duck_config);
    let duck: Duck = serde_json::from_reader(duck_reader)?;

    let emoji_list = File::open("database/all_emoji.json")?;
    let emoji_reader = BufReader::new(emoji_list);
    let emoji_set: HashSet<String> = serde_json::from_reader(emoji_reader)?;

    let emoji_mode_file = File::open("database/emoji_mode.json")?;
    let emoji_mode_reader = BufReader::new(emoji_mode_file);
    let emoji_mode_set_raw: EmojiModeLog = serde_json::from_reader(emoji_mode_reader)?;

    let mut emoji_mode_set = HashSet::new();
    for user in emoji_mode_set_raw.users {
        emoji_mode_set.insert(EmojiModeEntity::User(user));
    }
    for channel in emoji_mode_set_raw.channels {
        emoji_mode_set.insert(EmojiModeEntity::Channel(channel));
    }

    let (sender, receiver) = std::sync::mpsc::channel();

    {
        let mut data = client.data.write();
        // Client Data (used to communicate between ducks)
        data.insert::<OtherDuck>(Arc::new(Mutex::new(sender)));
        data.insert::<DuckIdentity>(which_duck);

        // Server Data
        data.insert::<ServerId>(GuildId::from(duck.server.server_id));
        data.insert::<WelcomeChannelId>(ChannelId::from(duck.server.welcome_channel_id));
        data.insert::<RDDChannelId>(ChannelId::from(duck.server.rdd_channel_id));

        // Moderation tools
        data.insert::<EmojiModeStates>(emoji_mode_set);
        data.insert::<EmojiList>(emoji_set);

        // Miscellaenous Configuration Data
        data.insert::<DuckMessages>(duck.messages);
        data.insert::<QuackVec>(duck.quacks);
        data.insert::<AutoReacts>(duck.auto_reacts);
    }

    Ok((
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(false)
                    .prefixes(vec!["!", "-", "~", "\\", "="])
                    .on_mention(None)
            })
            .after(|_, _, command_name, error| {
                if let Err(why) = error {
                    eprintln!("Command '{}' returned error {:?}", command_name, why);
                }
            })
            .normal_message(|ctx, message| {
                if triggers::emoji_only::allowed_message(ctx, message) {
                    triggers::quack(ctx, message);
                    triggers::auto_react(ctx, message);
                }
            }),
        receiver,
    ))
}
