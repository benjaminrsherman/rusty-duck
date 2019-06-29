use serde::Deserialize;
use serenity::model::id::ChannelId;
use serenity::prelude::TypeMapKey;

#[derive(Deserialize)]
pub struct Duck {
    pub quacks: Vec<String>,
    pub rdd_channel_id: u64,
}

pub struct QuackVec;
impl TypeMapKey for QuackVec {
    type Value = Vec<String>;
}

pub struct RDDChannelId;
impl TypeMapKey for RDDChannelId {
    type Value = ChannelId;
}
