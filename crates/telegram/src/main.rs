use std::path::PathBuf;

use teloxide::{net::Download, prelude::*};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    println!("Iniciando Based Songs Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(audio) = msg.audio() {
            bot.send_message(msg.chat.id, "Baixando áudio").await?;
            let file = bot.get_file(audio.file.id.clone()).await?;
            let default_file_name = &String::from("input.mp3");
            let file_name = audio.file_name.as_ref().unwrap_or(default_file_name);
            let path = PathBuf::from("/tmp").join(file_name);
            let mut output = File::create(&path)
                .await
                .map_err(|e| teloxide::RequestError::from(std::sync::Arc::new(e)))?;
            bot.download_file(&file.path, &mut output).await?;
            bot.send_message(
                msg.chat.id,
                format!("Áudio baixado para: {}", path.display()),
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
