use teloxide::{prelude::*, utils::command::BotCommands};
use std::error::Error;
use url::Url;

const ANEKDOT_RANDOM_URL: &str = "https://www.anekdot.ru/random/anekdot/";
const ANEKDOT_SEARCH_URL: &str = "https://www.anekdot.ru/search/";
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

fn html_to_joke(html: String) -> String {
    let response_msg;
    let parts = html.split("<div class=\"text\">").collect::<Vec<&str>>();
    if parts.len() < 2 {
        response_msg = "Ой чот не парсится анекдод, всё сломалось ((";
    } else {
        let joke_start = parts[1];
        let parts = joke_start.split("</div>").collect::<Vec<&str>>();
        response_msg = parts[0];
    }

    return response_msg.replace("<br>", "").to_owned();
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
            let response_msg: String;
            if criteria.len() == 0 {
                response_msg = "Исползуйте так: /find текст поиска".to_owned();
            } else {
                let url = Url::parse_with_params(ANEKDOT_SEARCH_URL, &[
                    ("query", criteria), 
                    ("ch[j]", "on".to_owned()),
                    ("ch[s]", "on".to_owned()),
                    ("mode", "any".to_owned()),
                    ("xcnt", "20".to_owned()),
                    ("maxlen", "".to_owned()),
                    ("order", "0".to_owned()),
                ])?;

                let html: String = surf::get(url).recv_string().await?;
                response_msg = html_to_joke(html);
            }
            bot.send_message(message.chat.id, response_msg).await?
        }
        Command::Random => {
            let html = surf::get(ANEKDOT_RANDOM_URL).recv_string().await?;

            bot.send_message(
                message.chat.id,
                html_to_joke(html),
            )
            .await?
        }
    };

    Ok(())
}
