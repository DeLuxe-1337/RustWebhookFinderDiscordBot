use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use std::env;

#[group]
#[commands(webhookv2, help, setact)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _context: Context, msg: Message) {
        if msg.content.contains(">") {
            println!("{} has said the command '{}'", msg.author.name, msg.content);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(">"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

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
                            "webhookv2",
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
async fn webhookv2(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Not implemented yet!").await?;

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

        if let (Some(">setact"), Some(game)) = (args.next(), args.next()) {
            ctx.set_activity(Activity::playing(game)).await;
        }
    } else {
        msg.reply(ctx, "You are not whitelisted to use this command!")
            .await?;
    }

    Ok(())
}
