// template.rs
use super::super::datatypes::DuckMessages;
use super::super::utils;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Retrieves a man page"]
pub fn man(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let client = reqwest::Client::new();
    let (mut url, res) = match client
        .get("https://www.freebsd.org/cgi/man.cgi")
        .query(&[("query", &args.rest())])
        .build()
    {
        Ok(request) => (
            request.url().clone().into_string(),
            match client.execute(request) {
                Ok(mut response) => match response.text() {
                    Ok(text) => text,
                    Err(_) => String::from("Sorry, no data found for"),
                },
                Err(_) => String::from("Sorry, no data found for"),
            },
        ),
        Err(why) => return Err(CommandError(format!("{:?}", why))),
    };

    if res.contains("Sorry, no data found for")
        || res.contains("Empty input.  Please type a manual page and search again.")
    {
        let data = ctx.data.read();
        let message_map = data
            .get::<DuckMessages>()
            .expect("Expected DuckMessages in ShareMap");
        let mut invalid_man_msg = message_map
            .get("man_page_not_found_pre")
            .expect("Expected 'man_page_not_found_pre' in DuckMessages")
            .clone();
        invalid_man_msg.push_str(&args.rest());
        invalid_man_msg.push_str(
            message_map
                .get("man_page_not_found_post")
                .expect("Expected 'man_page_not_found_post' in DuckMessages"),
        );

        utils::delay_send(&ctx.http, &msg.channel_id, &invalid_man_msg, 4);
    } else {
        url.insert_str(0, "<");
        url.push_str(">");

        utils::delay_send(&ctx.http, &msg.channel_id, &url, 3);
    }

    Ok(())
}
