use teloxide::{prelude::*, utils::command::BotCommands};
use std::error::Error;

const ANEKDOT_URL: &str = "https://www.anekdot.ru/random/anekdot/";
const BOT_TOKEN: &str = "5289010987:AAH4ZP4-c71xJNzj4Mx1HTJuNnhRJzPoR4A";


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting a bot");
    let bot = Bot::new(BOT_TOKEN).auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}


#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Список поддерживаемых команд:")]
enum Command {
    #[command(description = "старт.")]
    Start,
    #[command(description = "помощь.")]
    Help,
    #[command(description = "найти анекдот.")]
    Find(String),
    #[command(description = "случайный анекдот.")]
    Random,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help | Command::Start => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Find(criteria) => {
            bot.send_message(message.chat.id, format!("Ниумею пока ((((( @{criteria}.")).await?
        }
        Command::Random => {
            let html = surf::get(ANEKDOT_URL).recv_string().await?;

            let response_msg;
            let parts = html.split("<div class=\"text\">").collect::<Vec<&str>>();
            if parts.len() < 2 {
                response_msg = "Ой чот не парсится анекдод, всё сломалось ((";
            } else {
                let anek_start = parts[1];
                let parts = anek_start.split("</div>").collect::<Vec<&str>>();
                response_msg = parts[0];
            }

            bot.send_message(
                message.chat.id,
                response_msg.replace("<br>", ""),
            )
            .await?
        }
    };

    Ok(())
}
