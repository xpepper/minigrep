use minigrep::Config;
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Cannot parse arguments: {err}");
        process::exit(1);
    });

    minigrep::run(&config).unwrap_or_else(|e| {
        eprintln!("Could not read file {}: {e}", config.file_path);
        process::exit(1)
    });
}
