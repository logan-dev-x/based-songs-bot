use std::path::PathBuf;

use lib_core::pitch_shift;
use teloxide::{net::Download, prelude::*};
use tokio::fs::{self, File};

#[tokio::main]
async fn main() {
    println!("Iniciando Based Songs Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(audio) = msg.audio() {
            bot.send_message(msg.chat.id, "⬇️ Baixando áudio...")
                .await?;

            let file = bot.get_file(audio.file.id.clone()).await?;

            let file_name = audio.file_name.as_deref().unwrap_or("input.mp3");

            let input_path = PathBuf::from("/tmp").join(file_name);

            let mut input_file = File::create(&input_path)
                .await
                .map_err(|e| teloxide::RequestError::from(std::sync::Arc::new(e)))?;

            bot.download_file(&file.path, &mut input_file).await?;
            drop(input_file);

            println!("Download concluído: {}", input_path.display());

            bot.send_message(msg.chat.id, "🎵 Convertendo para 432 Hz...")
                .await?;

            let output_path = PathBuf::from("/tmp").join(format!("converted-{}", file_name));

            pitch_shift(&input_path, &output_path, 440.0, 432.0).map_err(|e| {
                teloxide::RequestError::from(std::sync::Arc::new(std::io::Error::other(format!(
                    "{:?}",
                    e
                ))))
            })?;

            println!("Conversão concluída: {}", output_path.display());

            bot.send_message(msg.chat.id, "⬆️ Enviando áudio...")
                .await?;

            println!("Iniciando upload do áudio...");

            bot.send_audio(msg.chat.id, teloxide::types::InputFile::file(&output_path))
                .await?;

            println!("Upload do áudio concluído!");

            // Limpeza dos arquivos temporários.
            fs::remove_file(&input_path)
                .await
                .map_err(|e| teloxide::RequestError::from(std::sync::Arc::new(e)))?;

            fs::remove_file(&output_path)
                .await
                .map_err(|e| teloxide::RequestError::from(std::sync::Arc::new(e)))?;

            println!("Arquivos temporários removidos!");
        } else {
            bot.send_message(msg.chat.id, "Envie um arquivo de áudio MP3.")
                .await?;
        }

        respond(())
    })
    .await;
}
