// template.rs
use super::super::utils;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Description goes here"]
pub fn template(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult;
