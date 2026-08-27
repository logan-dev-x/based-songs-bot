use lib_core::pitch_shift;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    match pitch_shift(&args[1], &args[2], 440.0, 432.0) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Error: {:?}", error);
            exit(1);
        }
    }
}
