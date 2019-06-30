// echo.rs
use super::super::utils;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Echoes the given message"]
pub fn echo(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let content = args.rest();

    if content.len() > 0 {
        utils::delay_send(&ctx.http, &msg.channel_id, content, 1);
    }

    Ok(())
}
