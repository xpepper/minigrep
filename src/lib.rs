use crate::CaseMode::CaseSensitive;
use CaseMode::CaseInsensitive;
use std::error::Error;
use std::{env, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    search(&config.query, &content, &config.case_mode)
        .iter()
        .for_each(|matching_string| println!("Found {matching_string}"));

    Ok(())
}

fn search<'a>(query: &str, content: &'a str, case_mode: &CaseMode) -> Vec<&'a str> {
    content
        .lines()
        .filter(|l| match case_mode {
            CaseSensitive => l.contains(query),
            CaseInsensitive => l.to_lowercase().contains(&query.to_lowercase()),
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
    pub fn build(mut arguments: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        arguments.next();

        let query = arguments.next().ok_or("query string not supplied")?;
        let file_path = arguments.next().ok_or("file path not supplied")?;
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
