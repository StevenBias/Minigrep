use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            Ok(Config { query: args[1].clone(), filename: args[2].clone() })
        } else {
            return Err("There are not enough arguments")
        }
    }
}

// dyn Error => Dynamix error to not avoid to declare the type of error
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
                        Rust:
                        safe, fast, productive.
                        Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
