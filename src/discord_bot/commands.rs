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
#[commands(webhookv2, help, setact)]
pub struct General;

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
