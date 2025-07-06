use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let _content =
        fs::read_to_string(&config.file_path).inspect(|c| println!("File content: {c}"))?;
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(arguments: &[String]) -> Result<Self, &'static str> {
        if arguments.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = arguments[1].clone();
        let file_path = arguments[2].clone();
        Ok(Config { query, file_path })
    }
}
