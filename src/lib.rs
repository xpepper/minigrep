use std::error::Error;
use std::fs;
use CaseMode::CaseInsensitive;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    search(&config.query, &content, CaseInsensitive)
        .iter()
        .for_each(|r| println!("Found {r}"));

    Ok(())
}

fn search<'a>(query: &str, content: &'a str, case_mode: CaseMode) -> Vec<&'a str> {
    content
        .lines()
        .filter(|l| match case_mode {
            CaseInsensitive => l.contains(query),
        })
        .collect::<Vec<_>>()
}

enum CaseMode {
    CaseInsensitive,
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
    use super::*;

    #[test]
    fn finds_a_case_sensitive_matching_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results = search(query, content, CaseInsensitive);

        assert_eq!(vec!["safe, fast, productive."], results)
    }

    #[test]
    #[ignore]
    fn finds_case_sensitive_matching_result() {
        let query = "RuSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let results = search(query, content, CaseInsensitive);

        assert_eq!(vec!["Rust:"], results);
    }
}
