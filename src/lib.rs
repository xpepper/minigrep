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

#[cfg(test)]
mod tests {
    #[test]
    fn finds_a_matching_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        let results = search(query, content);

        assert_eq!(vec!["safe, fast, productive."], results)
    }

    fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        content
            .lines()
            .filter(|l| l.contains(query))
            .collect::<Vec<_>>()
    }
}
