use std::path::PathBuf;

use lib_core::pitch_shift;
use teloxide::{net::Download, prelude::*};
use tokio::fs::{self, File};

async fn report_error(
    bot: &Bot,
    chat_id: ChatId,
    stage: &str,
    error: impl std::fmt::Debug,
) -> ResponseResult<()> {
    eprintln!("Erro durante {}: {:?}", stage, error);

    bot.send_message(
        chat_id,
        format!("❌ Não foi possível processar o áudio.\nEtapa: {}", stage),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Iniciando Based Songs Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let Some(audio) = msg.audio() else {
            bot.send_message(msg.chat.id, "Envie um arquivo de áudio MP3.")
                .await?;
            return respond(());
        };

        bot.send_message(msg.chat.id, "⬇️ Baixando áudio...")
            .await?;

        let file = match bot.get_file(audio.file.id.clone()).await {
            Ok(file) => file,
            Err(err) => {
                report_error(&bot, msg.chat.id, "obtenção do arquivo", err).await?;
                return respond(());
            }
        };

        let file_name = audio.file_name.as_deref().unwrap_or("input.mp3");

        let input_path = PathBuf::from("/tmp").join(file_name);

        let mut input_file = match File::create(&input_path).await {
            Ok(file) => file,
            Err(err) => {
                report_error(&bot, msg.chat.id, "criação de arquivo temporário", err).await?;
                return respond(());
            }
        };

        if let Err(error) = bot.download_file(&file.path, &mut input_file).await {
            report_error(&bot, msg.chat.id, "download", error).await?;
            return respond(());
        }
        drop(input_file);

        println!("Download concluído: {}", input_path.display());

        bot.send_message(msg.chat.id, "🎵 Convertendo para 432 Hz...")
            .await?;

        let output_path = PathBuf::from("/tmp").join(format!("converted-{}", file_name));

        if let Err(error) = pitch_shift(&input_path, &output_path, 440.0, 432.0) {
            report_error(&bot, msg.chat.id, "conversão", error).await?;
            return respond(());
        }
        println!("Conversão concluída: {}", output_path.display());

        bot.send_message(msg.chat.id, "⬆️ Enviando áudio...")
            .await?;

        println!("Iniciando upload do áudio...");

        if let Err(error) = bot
            .send_audio(msg.chat.id, teloxide::types::InputFile::file(&output_path))
            .await
        {
            report_error(&bot, msg.chat.id, "envio do áudio", error).await?;
        }

        println!("Upload do áudio concluído!");

        if let Err(error) = fs::remove_file(&input_path).await {
            eprintln!(
                "Aviso: não foi possível remover {:?}: {:?}",
                input_path, error
            );
        }

        if let Err(error) = fs::remove_file(&output_path).await {
            eprintln!(
                "Aviso: não foi possível remover {:?}: {:?}",
                output_path, error
            );
        }

        println!("Arquivos temporários removidos!");

        respond(())
    })
    .await;
}
