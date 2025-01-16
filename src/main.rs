use serde_json::{json, Value};
use std::io::{self, Write};
use tokio;
mod httpreq;
use httpreq::gemini_prompt;
mod settings;
use settings::{GEMINI_KEY,DELIM,NAME};

use azalea::{chat::*, pathfinder::goals::*, prelude::*, BlockPos};
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline(NAME);

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost:8080")
        .await
        .unwrap();

    //GEMINIPART
    println!("I'm sorry for this junk.\n\n");
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if m.username().unwrap() != bot.username() {
                let msg = m.content();
                let plyr = m.username().unwrap();

                println!("{}!!!", plyr);

                //prompt thing
                match gemini_prompt(GEMINI_KEY, &msg).await {
                    Ok(response) => {
                        bot.chat(&response.replace(DELIM, "").as_str());
                    }
                    Err(e) => bot.chat(format!("Error1: {}", e).as_str()),
                }
                //logic idk
                if msg == "!ping".to_string() {
                    bot.goto(BlockPosGoal(BlockPos::new(10, -55, 10)));
                    bot.chat("pong!");
                }
                if msg == "freeze!".to_string() {
                    bot.stop_pathfinding();

                    bot.chat("okay!");
                }
                if msg == "!data".to_string() {
                    bot.stop_pathfinding();
                    bot.chat("SHUT UP!");
                    println!("{:?}", bot.tab_list());
                }
            }
        }
        _ => {}
    }

    Ok(())
}
