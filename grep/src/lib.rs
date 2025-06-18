use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.string, &contents)
    } else {
        search_case_insensitive(&config.string, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub string: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let string = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{string, filename, case_sensitive})
    }
}

pub fn search<'a>(string: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(string) {
            results.push(line);
        }
    }
    results
}

// need to handle the cases where a user want to find the same string in lowercase,
// so it would automatically lowercase all of the lines, and check if the resulting
// line conatins the wanted one.
pub fn search_case_insensitive<'a>(string: &str, contents: &'a str) -> Vec<&'a str> {
    let string = string.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&string) {
            results.push(line);
        }
    }
    results
}

// writing a failing test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let (string, contents) = ("duct", "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.");
        assert_eq!(
            vec!["safe, fast, productive."],
            search(string, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let (string, contents) = (
"rUsT",
"\
Rust:
safe, fast, productive.
Pick three.
Trust me."
            );
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(string, contents)
        );
    }
}

