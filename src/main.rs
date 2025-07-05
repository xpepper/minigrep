use std::{env, fs};

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let (_query, file_path) = parse_config(&arguments);

    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| panic!("Could not read file {}: {}", file_path, e));

    println!("{content}");
}

fn parse_config(arguments: &[String]) -> (&str, &str) {
    let query = &arguments[1];
    let file_path = &arguments[2];
    (query, file_path)
}
