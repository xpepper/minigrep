use minigrep::Config;
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Cannot parse arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    minigrep::run(&config).unwrap_or_else(|e| {
        println!("Could not read file {}: {e}", config.file_path);
        process::exit(1)
    });
}
