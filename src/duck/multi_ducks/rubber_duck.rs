use super::super::triggers::*;
use super::super::*;
use super::both_ducks_init;

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
        guild::Member,
        id::{ChannelId, GuildId, UserId},
    },
    utils::MessageBuilder,
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
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

group!({
    name: "general",
    help_name: "General commands",
    options: {
        checks: [EmojiMode],
    },
    commands: [ai, code, echo, lmdtfy, lmgtfy, man, tex]
});

group!({
    name: "admin",
    help_name: "admin",
    options: {
        required_permissions: [ADMINISTRATOR],
    },
    commands: [emoji]
});

pub fn init_client(
    client: &mut Client,
    which_duck: Identity,
) -> Result<Receiver<(ChannelId, String, usize)>, Box<Error>> {
    let (framework, receiver) = both_ducks_init(client, which_duck)?;

    client.with_framework(
        framework
            .help(&HELP_CMD)
            .group(&GENERAL_GROUP)
            .group(&ADMIN_GROUP),
    );

    Ok(receiver)
}

// EVENT HANDLER
pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        let data = ctx.data.read();
        let server_id = data
            .get::<ServerId>()
            .expect("Expected ServerId in ShareMap");

        if server_id != &guild_id {
            return;
        }

        // Send a public greeting
        let welcome_channel_id = data
            .get::<WelcomeChannelId>()
            .expect("Expected WelcomeChannelId in ShareMap");

        let messages_hashmap = data
            .get::<DuckMessages>()
            .expect("Expected DuckMessages in ShareMap");

        let welcome_message_public_pre = messages_hashmap
            .get("welcome_public_pre_mention")
            .expect("Expected 'welcome_public_pre_mention' in DuckMessages");
        let welcome_message_public_post = messages_hashmap
            .get("welcome_public_post_mention")
            .expect("Expected 'welcome_public_post_mention' in DuckMessages");

        let welcome_message_public = MessageBuilder::new()
            .push(welcome_message_public_pre)
            .mention(&new_member)
            .push(welcome_message_public_post)
            .build();
        if let Err(why) = welcome_channel_id.say(&ctx.http, welcome_message_public) {
            eprintln!("Error sending message: {:?}", why);
        }

        // Send private instructions on how the server works
        let welcome_message_dm = messages_hashmap
            .get("welcome_dm")
            .expect("Expected 'welcome_dm' in DuckMessages");

        let member = match new_member.user_id().to_user(&ctx) {
            Ok(mbr) => mbr,
            Err(why) => {
                eprintln!(
                    "Error parsing user {} when they entered the server: {:?}",
                    new_member.display_name(),
                    why
                );
                return;
            }
        };

        if let Err(why) = member.direct_message(&ctx, |m| m.content(&welcome_message_dm)) {
            eprintln!(
                "Error sending DM to user {}: {:?}",
                new_member.display_name(),
                why
            );
        }
    }
}
