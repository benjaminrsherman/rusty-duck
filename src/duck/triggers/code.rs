// code.rs
use super::super::duck::*;
use super::super::utils;
use serenity::framework::standard::{macros::command, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Gives some information about the code running the ducks"]
pub fn code(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let content_rd = data
        .get::<DuckMessages>()
        .expect("Expected DuckMessages in ShareMap")
        .get("code_rd")
        .expect("Expected 'code_rd' in DuckMessage");

    utils::delay_send(&ctx.http, &msg.channel_id, content_rd, 1);

    let content_rm = data
        .get::<DuckMessages>()
        .expect("Expected DuckMessages in ShareMap")
        .get("code_rm")
        .expect("Expected 'code_rm' in DuckMessage");
    let rm_sender = data
        .get::<OtherDuck>()
        .expect("Expected OtherDuck in ShareMap");

    let rm_sender = match rm_sender.lock() {
        Ok(sender) => sender,
        Err(why) => {
            let why = format!("{:?}", why);
            return Err(CommandError(why));
        }
    };

    if let Err(why) = rm_sender.send((msg.channel_id, content_rm.to_string(), 1)) {
        let why = format!("{:?}", why);
        return Err(CommandError(why));
    }

    Ok(())
}
