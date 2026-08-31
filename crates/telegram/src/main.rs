use std::env;
use std::process::Stdio;
use tokio::process::Command;

use axum::{Router, routing::get};
use lib_core::pitch_shift;
use teloxide::{net::Download, prelude::*, types::InputFile};
use tempfile::tempdir;
use tokio::fs::File;

async fn normalize_audio(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-i",
        ])
        .arg(input_path)
        .args([
            "-vn",
            "-codec:a",
            "libmp3lame",
            "-q:a",
            "2",
        ])
        .arg(output_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(format!(
            "FFmpeg falhou: {}",
            stderr.trim()
        )
        .into());
    }

    Ok(())
}

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

async fn health_check() -> &'static str {
    "OK"
}

async fn run_bot() {
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
            Err(error) => {
                report_error(&bot, msg.chat.id, "obtenção do arquivo", error).await?;

                return respond(());
            }
        };

        let file_name = audio.file_name.as_deref().unwrap_or("input.mp3");

        // Cria um diretório temporário exclusivo para esta conversão.
        let temp_dir = match tempdir() {
            Ok(dir) => dir,
            Err(error) => {
                report_error(&bot, msg.chat.id, "criação do diretório temporário", error).await?;

                return respond(());
            }
        };

        let input_path = temp_dir.path().join(file_name);

        let mut input_file = match File::create(&input_path).await {
            Ok(file) => file,
            Err(error) => {
                report_error(&bot, msg.chat.id, "criação do arquivo temporário", error).await?;

                return respond(());
            }
        };

        if let Err(error) = bot.download_file(&file.path, &mut input_file).await {
            report_error(&bot, msg.chat.id, "download", error).await?;

            return respond(());
        }

        drop(input_file);

        println!("Download concluído: {}", input_path.display());


let normalized_path = temp_dir.path().join("normalized.mp3");

if let Err(error) = normalize_audio(&input_path, &normalized_path).await {
    report_error(
        &bot,
        msg.chat.id,
        "normalização do áudio",
        error,
    )
    .await?;

    return respond(());
}

println!(
    "Áudio normalizado: {}",
    normalized_path.display()
);

bot.send_message(msg.chat.id, "🎵 Convertendo para 432 Hz...")
    .await?;

let output_path = temp_dir
    .path()
    .join(format!("converted-{}", file_name));

if let Err(error) = pitch_shift(
    &normalized_path,
    &output_path,
    440.0,
    432.0,
) {
    report_error(
        &bot,
        msg.chat.id,
        "conversão",
        error,
    )
    .await?;

    return respond(());
}





        println!("Conversão concluída: {}", output_path.display());

        bot.send_message(msg.chat.id, "⬆️ Enviando áudio...")
            .await?;

        if let Err(error) = bot
            .send_audio(msg.chat.id, InputFile::file(&output_path))
            .await
        {
            report_error(&bot, msg.chat.id, "envio do áudio", error).await?;

            return respond(());
        }

        println!("Áudio enviado!");

        // Ao sair daqui, temp_dir é destruído.
        // Os arquivos temporários são removidos automaticamente.
        drop(temp_dir);

        println!("Arquivos temporários removidos!");

        respond(())
    })
    .await;
}

#[tokio::main]
async fn main() {
    println!("Iniciando Based Songs Bot...");

    let port = env::var("PORT").expect("PORT não definida");
    let addr = format!("0.0.0.0:{port}");

    let app = Router::new().route("/", get(health_check));

    let http_server = async {
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Falha ao iniciar servidor HTTP");

        println!("Health check disponível em http://{addr}");

        axum::serve(listener, app)
            .await
            .expect("Servidor HTTP encerrou com erro");
    };

    tokio::join!(run_bot(), http_server);
}
