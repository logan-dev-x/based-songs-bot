use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    println!("Iniciando Based Songs Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(audio) = msg.audio() {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Áudio recebido!\nNome: {}\nTamanho: {} bytes",
                    audio.file_name.as_deref().unwrap_or("desconhecido"),
                    audio.file.size,
                ),
            )
            .await?;
        } else {
            bot.send_message(msg.chat.id, "Envie um arquivo de áudio MP3.")
                .await?;
        }

        respond(())
    })
    .await;
}
