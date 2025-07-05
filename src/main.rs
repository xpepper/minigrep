use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let _content = fs::read_to_string(config.file_path.clone())
        .unwrap_or_else(|e| panic!("Could not read file {}: {}", config.file_path, e));
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(arguments: &[String]) -> Self {
        let query = arguments[1].clone();
        let file_path = arguments[2].clone();
        Config { query, file_path }
    }
}
