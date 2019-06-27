use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod utils;
use utils::*;

pub struct DuckHandler {
    quacks: Vec<String>,
}

impl DuckHandler {
    /// Initialize the duck with values from the environment
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file = File::open("database/quacks.json")?;

        let buf_reader = BufReader::new(file);
        let quacks = serde_json::from_reader(buf_reader)?;

        Ok(DuckHandler { quacks })
    }
}

impl EventHandler for DuckHandler {
    /// Respond to a message with a random quack.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let quack = &self.quacks[rand_range(0, self.quacks.len())];
        if let Err(why) = msg.channel_id.say(&ctx.http, quack) {
            println!("Error sending message: {:?}", why);
        }
    }

    /// Print console output when the bot has connected.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
