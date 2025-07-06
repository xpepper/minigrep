use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Cannot parse arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let _content = fs::read_to_string(config.file_path.clone())
        .inspect(|c| println!("File content: {c}"))
        .unwrap_or_else(|e| {
            println!("Could not read file {}: {e}", config.file_path);
            process::exit(1)
        });
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
