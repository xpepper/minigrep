use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = parse_config(&args);

    println!("[LOG] Searching for {}", config.query);
    println!("[LOG] In file {}", config.file_path);

    let content = fs::read_to_string(config.file_path.clone())
        .unwrap_or_else(|e| panic!("Could not read file {}: {}", config.file_path, e));

    println!("{content}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(arguments: &[String]) -> Config {
    let query = arguments[1].clone();
    let file_path = arguments[2].clone();
    Config { query, file_path }
}
