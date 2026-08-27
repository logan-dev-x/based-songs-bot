use clap::Parser;
use lib_core::{decode::decode, encode::encode, pitch::pitch};
use std::process::exit;

#[derive(Parser)]
#[command(name = "based-songs")]
#[command(about = "Converte o pitch de arquivos de áudio")]
struct Args {
    input: String,
    output: String,
    #[arg(long, default_value_t = 440.0)]
    from: f64,
    #[arg(long, default_value_t = 432.0)]
    to: f64,
}

fn main() {
    let args = Args::parse();

    println!("Lendo e decodificando arquivo...");

    let audio = match decode(&args.input) {
        Ok(audio) => audio,
        Err(error) => {
            eprintln!("Erro ao decodificar: {:?}", error);
            exit(1);
        }
    };

    println!("Aplicando pitch-shift de 440Hz para 432Hz...");

    let audio = match pitch(&audio, args.from, args.to) {
        Ok(audio) => audio,
        Err(error) => {
            eprintln!("Erro no pitch-shift: {:?}", error);
            exit(1);
        }
    };

    println!("Codificando o novo áudio...");

    if let Err(error) = encode(&audio, &args.output) {
        eprintln!("Erro ao codificar: {:?}", error);
        exit(1);
    }

    println!("Concluído com sucesso!");
}
