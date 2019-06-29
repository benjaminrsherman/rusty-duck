// academic_integrity.rs
use super::super::utils;
use super::super::DuckMessages;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Sends a reminder about RPI's academic integrity policy"]
#[aliases("academic_integrity")]
pub fn ai(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let ai_message = data
        .get::<DuckMessages>()
        .expect("Expected DuckMessages in ShareMap")
        .get("academic_integrity")
        .expect("Expected 'academic_integrity' in ShareMap");

    utils::delay_send(ctx, &msg.channel_id, ai_message, 2);

    Ok(())
}
