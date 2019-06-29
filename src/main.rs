use serenity::prelude::*;

mod duck;

fn main() {
    // Read the bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, duck::Handler).expect("Err creating client");
    if let Err(why) = duck::init_client(&mut client) {
        println!("Error initializing client: {:?}", why);
    }

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
