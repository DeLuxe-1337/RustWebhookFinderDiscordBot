use std::env;

use super::commands::*;

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

pub async fn start() {
    println!("Launching bot");
    
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
        println!("Client error: {:?}", why);
    }
}
