// template.rs
use super::super::utils;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Generates a lmdtfy link"]
pub fn lmdtfy(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let client = reqwest::Client::new();
    let mut res = match client
        .get("https://lmgtfy.com")
        .query(&[("s", "d"), ("q", &args.rest())])
        .build()
    {
        Ok(request) => request.url().clone().into_string(),
        Err(why) => return Err(CommandError(format!("{:?}", why))),
    };
    res.insert_str(0, "<");
    res.push_str(">");

    utils::delay_send(&ctx.http, &msg.channel_id, &res, 3);

    Ok(())
}

#[command]
#[description = "Generates a lmgtfy link"]
pub fn lmgtfy(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let client = reqwest::Client::new();
    let mut res = match client
        .get("https://lmgtfy.com")
        .query(&[("q", &args.rest())])
        .build()
    {
        Ok(request) => request.url().clone().into_string(),
        Err(why) => return Err(CommandError(format!("{:?}", why))),
    };
    res.insert_str(0, "<");
    res.push_str(">");

    utils::delay_send(&ctx.http, &msg.channel_id, &res, 3);

    Ok(())
}
