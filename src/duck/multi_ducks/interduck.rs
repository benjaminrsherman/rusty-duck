// interduck.rs
use super::super::utils;

use serenity::{http::raw::Http, model::id::ChannelId};
use std::sync::{mpsc::Receiver, Arc};

pub fn interduck_communication(http: Arc<Http>, receiver: Receiver<(ChannelId, String, usize)>) {
    loop {
        let (channel, content, delay_scale) =
            receiver.recv().expect("Unable to receive from channel!");

        utils::delay_send(&http, &channel, &content, delay_scale);
    }
}
