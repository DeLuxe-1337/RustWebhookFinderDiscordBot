pub use serenity::async_trait;
pub use serenity::client::{Client, Context, EventHandler};
pub use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
pub use serenity::model::channel::Message;
pub use serenity::model::gateway::Activity;
pub use serenity::model::gateway::Ready;

#[group]
#[commands(scan, help, setact)]
pub struct General;

use std::io;
use std::fs::File;
use std::io::Write;

use super::webhook_detect::get_discord_message;
use rust_strings::{Encoding};
use reqwest::*;
use std::io::copy;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Help command")
                    .fields(vec![
                        ("help", "Returns a list of commands", false),
                        (
                            "scan",
                            "[file or file url] scans the file for webhooks",
                            false,
                        ),
                        (
                            ":warning: whitelisted commands :warning:",
                            "You must be whitelisted to use the following commands",
                            false,
                        ),
                        (
                            "setact",
                            "[string] sets the activity to inputted str",
                            false,
                        ),
                    ])
                    .footer(|f| f.text("The prefix is >"))
            })
        })
        .await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn string_dump(ctx: &Context, msg: &Message) -> CommandResult {


    Ok(())
 }

#[command]
async fn scan(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = msg.content.split_whitespace();

    args.next();

    if let Some(url) = args.next() {
        if url.contains("http") {
            let mut resp = reqwest::get(url).await?;
            let mut out = File::create("result.exe").expect("failed to create file");
            io::copy(&mut resp.text().await?.as_bytes(), &mut out).expect("failed to copy content");

            let result = get_discord_message("result.exe".to_string());

            msg.reply(ctx, result).await?;
        }

        return Ok(());
    }

    let attach = &msg.attachments[0];

    if !attach.filename.contains(".exe") {
        msg.reply(ctx, "Not an exe file!").await?;
        return Ok(());
    }

    if let Ok(content) = attach.download().await {
        let mut file = File::create(attach.filename.clone()).unwrap();
        file.write_all(&content).unwrap();

        let result = get_discord_message(attach.filename.clone());

        msg.reply(ctx, result).await?;
    }

    Ok(())
}

async fn is_whitelisted(id: u64) -> bool {
    let whitelisted = vec![271389338531528714];

    for i in whitelisted {
        if i == id {
            return true;
        }
    }

    return false;
}

#[command]
async fn setact(ctx: &Context, msg: &Message) -> CommandResult {
    if is_whitelisted(msg.author.id.0).await {
        let mut args = msg.content.splitn(2, ' ');

        args.next();

        if let Some(activity) = args.next() {
            ctx.set_activity(Activity::playing(activity)).await;

            msg.reply(ctx, "Activity set!").await?;
        }
    } else {
        msg.reply(ctx, "You are not whitelisted to use this command!")
            .await?;
    }

    Ok(())
}
