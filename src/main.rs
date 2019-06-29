mod duck;
use duck::interduck::interduck_communication;
use duck::*;

use serenity::prelude::*;

fn main() {
    // Read the bot token from the environment
    let rd_token = std::env::var("DISCORD_TOKEN_RD")
        .expect("Expected a token for Rubber Duck in the environment");
    let rm_token = std::env::var("DISCORD_TOKEN_RM")
        .expect("Expected a token for Robo-Mallard in the environment");

    let mut rd_client = Client::new(&rd_token, rubber_duck::Handler).expect("Err creating client");
    let mut rm_client = Client::new(&rm_token, robo_mallard::Handler).expect("Err creating client");
    let rm_receiver = rubber_duck::init_client(&mut rd_client, duck::Identity::RubberDuck)
        .expect("Error initializing Rubber Duck");
    let rd_receiver = robo_mallard::init_client(&mut rm_client, duck::Identity::RoboMallard)
        .expect("Error initializing Robo-Mallard");

    let rd_http = rd_client.cache_and_http.http.clone();
    let rm_http = rm_client.cache_and_http.http.clone();

    std::thread::spawn(move || interduck_communication(rd_http, rd_receiver));
    std::thread::spawn(move || interduck_communication(rm_http, rm_receiver));

    std::thread::spawn(move || {
        if let Err(why) = rd_client.start() {
            println!("Rubber Duck error: {:?}", why);
        }
    });

    if let Err(why) = rm_client.start() {
        println!("Robo-Mallard error: {:?}", why);
    }
}
