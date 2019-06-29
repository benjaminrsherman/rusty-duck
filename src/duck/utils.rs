use getrandom::getrandom;
use serenity::http::raw::Http;
use serenity::model::id::ChannelId;

/// Returns a random i64 in the range `[start, end)`
#[inline]
pub fn rand_range(start: usize, end: usize) -> usize {
    let mut bytes = [0u8; 8]; // Hopefully we're running on 64-bit
    getrandom(&mut bytes).unwrap_or_default();

    let rnd: usize = unsafe { std::mem::transmute(bytes) };

    rnd % (end - start) + start
}

pub fn delay_send(http: &Http, channel: &ChannelId, content: &str, delay_scale: usize) {
    let mut delay = 1000 + 10 * content.len() / delay_scale;

    while delay > 5000 {
        if let Err(why) = channel.broadcast_typing(http) {
            eprintln!("Error broadcasting typing: {:?}", why);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5000));
        delay -= 5000;
    }
    if let Err(why) = channel.broadcast_typing(http) {
        eprintln!("Error broadcasting typing: {:?}", why);
    }
    std::thread::sleep(std::time::Duration::from_millis(delay as u64));

    if let Err(why) = channel.say(http, content) {
        eprintln!("Error sending message: {:?}", why);
    }
}
