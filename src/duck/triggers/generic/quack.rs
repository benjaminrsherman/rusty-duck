// generic.rs
use super::super::super::datatypes::*;
use super::super::super::utils;
use serenity::{model::prelude::Message, prelude::Context};

fn should_quack(ctx: &Context, msg: &Message) -> bool {
    if msg.author.bot {
        return false;
    }

    let bot_id = match ctx.http.get_current_user() {
        Ok(usr) => usr.id,
        Err(why) => {
            eprintln!("Error getting current_user id: {:?}", why);
            return false;
        }
    };

    let data = ctx.data.read();
    let rdd_channel_id = data
        .get::<RDDChannelId>()
        .expect("Expected RDDChannel in ShareMap");
    let identity = data
        .get::<DuckIdentity>()
        .expect("Expected DuckIdentity in ShareMap");

    (&msg.channel_id == rdd_channel_id && identity == &Identity::RubberDuck)
        || msg.is_private()
        || msg.mentions_user_id(bot_id)
}

pub fn quack(ctx: &Context, msg: &Message) {
    if !should_quack(ctx, msg) {
        return;
    }

    let data = ctx.data.read();
    let quacks = data
        .get::<QuackVec>()
        .expect("Expected QuackVec in ShareMap");

    let idx = utils::rand_range(0, quacks.len());

    utils::delay_send(&ctx.http, &msg.channel_id, &quacks[idx], 1);
}
