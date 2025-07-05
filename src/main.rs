use std::{env, fs};

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let _query = &arguments[1];
    let file_path = &arguments[2];

    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| panic!("Could not read file {}: {}", file_path, e));

    println!("{content}");
}
