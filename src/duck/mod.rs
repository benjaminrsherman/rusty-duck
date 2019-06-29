pub mod utils;

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
        id::{ChannelId, UserId},
    },
};
use std::collections::HashSet;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod duck;
use duck::*;

// EVENT HANDLER
pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// LOAD TRIGGERS
mod triggers;
use triggers::echo::*;

#[help]
#[individual_command_tip = "Hello!\n\
If you want more information about a specific command, just pass the command as an argument."]
#[command_not_found_text = "Could not find `{}`."]
#[max_levenshtein_distance(3)]
fn help_cmd(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

group!({
    name: "general",
    commands: [echo]
});

pub fn init_client(client: &mut Client) -> Result<(), Box<Error>> {
    let file = File::open("database/duck.json")?;
    let reader = BufReader::new(file);

    let duck: Duck = serde_json::from_reader(reader)?;

    {
        let mut data = client.data.write();
        data.insert::<QuackVec>(duck.quacks);
        data.insert::<RDDChannelId>(ChannelId::from(duck.rdd_channel_id));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(false)
                    .prefixes(vec!["!", "-", "~", "\\"])
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

    Ok(())
}
