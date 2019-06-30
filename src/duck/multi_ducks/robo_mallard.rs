use super::super::triggers;
use super::super::triggers::*;
use super::super::*;

use serenity::prelude::*;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
    },
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, GuildId, UserId},
    },
};
use std::collections::HashSet;
use std::sync::{mpsc::Receiver, Arc, Mutex};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[help]
#[individual_command_tip = "Hello!\n\
If you want more information about a specific command, just pass the command as an argument."]
#[command_not_found_text = "Could not find `{}`."]
#[max_levenshtein_distance(3)]
#[strikethrough_commands_tip_in_guild(" ")]
#[strikethrough_commands_tip_in_dm(" ")]
fn help_cmd(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    if msg.is_private() {
        // only respond to help in dms
        return help_commands::with_embeds(ctx, msg, args, help_options, groups, owners);
    }

    Ok(())
}

group!({
    name: "general",
    help_name: "general",
    options: {
        only_in: "dms",
    },
    commands: [ai, code, echo]
});

pub fn init_client(
    client: &mut Client,
    which_duck: Identity,
) -> Result<Receiver<(ChannelId, String, usize)>, Box<Error>> {
    let file = File::open("database/duck.json")?;
    let reader = BufReader::new(file);

    let duck: Duck = serde_json::from_reader(reader)?;

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

        // Miscellaenous Configuration Data
        data.insert::<DuckMessages>(duck.messages);
        data.insert::<QuackVec>(duck.quacks);
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(false)
                    .prefixes(vec!["!", "-", "~", "\\", "="])
                    .on_mention(None)
            })
            .after(|_, _, command_name, error| {
                if let Err(why) = error {
                    println!("Command '{}' returned error {:?}", command_name, why);
                }
            })
            .normal_message(|ctx, message| {
                triggers::quack(ctx, message);
            })
            .help(&HELP_CMD)
            .group(&GENERAL_GROUP),
    );

    Ok(receiver)
}

// EVENT HANDLER
pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
