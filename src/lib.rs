use std::env;
use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() > 2 {
            // The first arg is the prog name, ignore it!
            args.next();

            let query = match args.next() {
                Some(q) => q,
                None => return Err("Error on the query string"),
            };
            let filename = match args.next() {
                Some(f) => f,
                None => return Err("Error on the file name"),
            };

            let case_sensitive = env::var("CASE_SENSITIVE").is_err();

            Ok(Config { query, filename, case_sensitive })
        } else {
            return Err("There are not enough arguments")
        }
    }
}

// dyn Error => Dynamix error to not avoid to declare the type of error
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search_case_insensitive(&config.query, &content) 
    } else {
        search(&config.query, &content) 
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

// Use en explicit lifetime 'a to specify that this is the content argument lifetime
// which is connected to the return Vec
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    return res;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
