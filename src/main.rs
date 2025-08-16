use minigrep_tool_cli::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Cannot parse arguments: {err}");
        process::exit(1);
    });

    minigrep_tool_cli::run(&config).unwrap_or_else(|e| {
        eprintln!("Could not read file {}: {e}", config.file_path);
        process::exit(1)
    });
}
