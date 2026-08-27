use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    println!("Iniciando Based Songs Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, "Olá!").await?;

        respond(())
    })
    .await;
}
