use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Cannot parse arguments: {err}");
        process::exit(1);
    });

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
    fn build(arguments: &[String]) -> Result<Self, &'static str> {
        if arguments.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = arguments[1].clone();
        let file_path = arguments[2].clone();
        Ok(Config { query, file_path })
    }
}
