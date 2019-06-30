use serde::Deserialize;
use serenity::model::id::{ChannelId, EmojiId, GuildId};
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::{mpsc::Sender, Arc, Mutex};

// Used to keep track of which duck is which
#[derive(PartialEq, Eq)]
pub enum Identity {
    RubberDuck,
    RoboMallard,
}

#[derive(Deserialize)]
pub struct AutoReaction {
    pub probability: usize, // Should be between 0 and 100
    pub emoji: Option<Vec<String>>,
    pub emotes: Option<Vec<EmojiId>>,
}

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
    pub messages: HashMap<String, String>,
    pub auto_reacts: HashMap<String, AutoReaction>,
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

pub struct DuckMessages;
impl TypeMapKey for DuckMessages {
    type Value = HashMap<String, String>;
}

pub struct OtherDuck;
impl TypeMapKey for OtherDuck {
    type Value = Arc<Mutex<Sender<(ChannelId, String, usize)>>>;
}

pub struct DuckIdentity;
impl TypeMapKey for DuckIdentity {
    type Value = Identity;
}

pub struct AutoReacts;
impl TypeMapKey for AutoReacts {
    type Value = HashMap<String, AutoReaction>;
}
