use serde::Deserialize;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::TypeMapKey;

#[derive(Deserialize)]
pub struct Server {
    pub server_id: u64,
    pub welcome_channel_id: u64,
    pub rdd_channel_id: u64,
}

#[derive(Deserialize)]
pub struct Duck {
    pub quacks: Vec<String>,
    pub server: Server,
}

pub struct QuackVec;
impl TypeMapKey for QuackVec {
    type Value = Vec<String>;
}

pub struct RDDChannelId;
impl TypeMapKey for RDDChannelId {
    type Value = ChannelId;
}

pub struct ServerId;
impl TypeMapKey for ServerId {
    type Value = GuildId;
}

pub struct WelcomeChannelId;
impl TypeMapKey for WelcomeChannelId {
    type Value = ChannelId;
}
