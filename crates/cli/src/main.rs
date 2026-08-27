use lib_core::pitch_shift;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("error");
        exit(1);
    }

    pitch_shift(&args[1], &args[2], 440.0, 432.0);
}
