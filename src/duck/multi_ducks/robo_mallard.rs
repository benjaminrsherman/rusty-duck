use super::super::triggers::*;
use super::super::*;

use serenity::prelude::*;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, UserId},
    },
};
use std::collections::HashSet;
use std::sync::mpsc::Receiver;

use std::error::Error;

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
    commands: [ai, code, echo, lmdtfy, lmgtfy, tex]
});

pub fn init_client(
    client: &mut Client,
    which_duck: Identity,
) -> Result<Receiver<(ChannelId, String, usize)>, Box<Error>> {
    let (framework, receiver) = both_ducks_init(client, which_duck)?;

    client.with_framework(framework.help(&HELP_CMD).group(&GENERAL_GROUP));

    Ok(receiver)
}

// EVENT HANDLER
pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
