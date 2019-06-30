// tex.rs
use super::super::super::utils;
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Renders LaTeX using latex2png.com"]
pub fn tex(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let content = args.rest();

    let form_params = [
        ("latex", content),
        ("res", "600"),
        ("color", "FFFFFF"),
        ("x", "62"),
        ("y", "28"),
    ];

    let reqwest_client = reqwest::Client::new();

    let res = reqwest_client
        .post("http://latex2png.com/")
        .form(&form_params)
        .send()?
        .text()?;

    let re = Regex::new("latex_(.*)png").unwrap();
    let captures = match re.captures(&res) {
        Some(cap) => cap,
        None => return Err(CommandError("Regex page parsing failed".to_string())),
    };

    let mut tex_link = String::from("http://latex2png.com/output//");
    tex_link.push_str(captures.get(0).unwrap().as_str());

    utils::delay_send(&ctx.http, &msg.channel_id, &tex_link, 3);

    Ok(())
}
