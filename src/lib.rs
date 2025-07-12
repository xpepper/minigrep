use crate::CaseMode::CaseSensitive;
use std::error::Error;
use std::{env, fs};
use CaseMode::CaseInsensitive;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    search(&config.query, &content, &config.case_mode)
        .iter()
        .for_each(|r| println!("Found {r}"));

    Ok(())
}

fn search<'a>(query: &str, content: &'a str, case_mode: &CaseMode) -> Vec<&'a str> {
    content
        .lines()
        .filter(|l| match case_mode {
            CaseSensitive => l.contains(query),
            CaseInsensitive => l.to_lowercase().contains(query.to_lowercase().as_str()),
        })
        .collect::<Vec<_>>()
}

enum CaseMode {
    CaseInsensitive,
    CaseSensitive,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub(crate) case_mode: CaseMode,
}
impl Config {
    pub fn build(arguments: &[String]) -> Result<Self, &'static str> {
        if arguments.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = arguments[1].clone();
        let file_path = arguments[2].clone();
        let case_mode = Self::case_mode()?;

        Ok(Config {
            query,
            file_path,
            case_mode,
        })
    }

    fn case_mode() -> Result<CaseMode, &'static str> {
        let case_mode = match env::var("CASE_MODE").as_deref().unwrap_or("sensitive") {
            "sensitive" => CaseSensitive,
            "insensitive" => CaseInsensitive,
            _ => return Err("Invalid case mode"),
        };
        Ok(case_mode)
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

        let results = search(query, content, &CaseSensitive);

        assert_eq!(vec!["safe, fast, productive."], results)
    }

    #[test]
    fn finds_case_sensitive_matching_result() {
        let query = "RuSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let results = search(query, content, &CaseInsensitive);

        assert_eq!(vec!["Rust:"], results);
    }
}
