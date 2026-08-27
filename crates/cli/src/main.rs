use lib_core::{decode::decode, encode::encode, pitch::pitch};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Uso: cli <entrada.mp3> <saida.mp3>");
        exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    println!("Lendo e decodificando arquivo...");

    let audio = match decode(input) {
        Ok(audio) => audio,
        Err(error) => {
            eprintln!("Erro ao decodificar: {:?}", error);
            exit(1);
        }
    };

    println!("Aplicando pitch-shift de 440Hz para 432Hz...");

    let audio = match pitch(&audio, 440.0, 432.0) {
        Ok(audio) => audio,
        Err(error) => {
            eprintln!("Erro no pitch-shift: {:?}", error);
            exit(1);
        }
    };

    println!("Codificando o novo áudio...");

    if let Err(error) = encode(&audio, output) {
        eprintln!("Erro ao codificar: {:?}", error);
        exit(1);
    }

    println!("Concluído com sucesso!");
}
