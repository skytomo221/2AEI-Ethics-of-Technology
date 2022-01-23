use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, fs};

#[derive(Serialize, Deserialize)]
struct Quiz {
    question: String,
    answer: String,
}

#[derive(Serialize, Deserialize)]
struct Quizzes {
    quizzes: Vec<Quiz>,
}

struct Handler {
    quizzes: Vec<Quiz>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let re = Regex::new(r"^q(\d+)$").unwrap();
        if re.is_match(&msg.content) {
            let caps = re.captures(&msg.content).unwrap();
            let seed = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let quiz = &self.quizzes[seed % self.quizzes.len()];
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("{}\n答え：||{}||", quiz.question, quiz.answer),
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let json = fs::read_to_string("quizzes.json").unwrap();
    let quizzes: Quizzes = serde_json::from_str(&json).unwrap();
    let quizzes = quizzes.quizzes;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler { quizzes })
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
