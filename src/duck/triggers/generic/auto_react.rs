// generic.rs
use super::super::super::duck::*;
use super::super::super::utils;
use serenity::{model::prelude::Message, prelude::Context};

pub fn auto_react(ctx: &Context, msg: &Message) {
    let data = ctx.data.read();
    let bot_id = data
        .get::<DuckIdentity>()
        .expect("Expected Identity in ShareMap");
    if bot_id == &Identity::RoboMallard && !msg.is_private() {
        return;
    };

    let auto_reacts = data
        .get::<AutoReacts>()
        .expect("Expected AutoReacts in ShareMap");

    for (key, auto_reaction) in auto_reacts {
        if msg.content.contains(key) {
            let rand_val = utils::rand_range(0, 100);

            if rand_val > auto_reaction.probability {
                continue;
            }

            if let Some(reactions) = &auto_reaction.emoji {
                for reaction in reactions {
                    if let Err(why) = msg.react(ctx, reaction.clone()) {
                        eprintln!("Error reacting to message: {:?}", why)
                    }
                }
            }

            if let Some(reactions) = &auto_reaction.emotes {
                match msg.guild(ctx) {
                    Some(guild) => {
                        let guild_emotes = &guild.read().emojis;

                        for emote in reactions {
                            let emote = match guild_emotes.get(emote) {
                                Some(emote) => emote,
                                None => {
                                    eprintln!(
                                        "Error finding {} in guild emote list, skipping",
                                        emote
                                    );
                                    continue;
                                }
                            };

                            if let Err(why) = msg.react(ctx, emote.clone()) {
                                eprintln!("Error reacting to message: {:?}", why)
                            }
                        }
                    }
                    None => continue,
                };
            }
        }
    }
}
