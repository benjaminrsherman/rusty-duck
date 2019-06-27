use serenity::prelude::Client;

mod duck;
use duck::DuckHandler;

fn main() {
    // Read the bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let handler = DuckHandler::new().expect("error creating handler");

    let mut client = Client::new(&token, handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
