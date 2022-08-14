use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = args.len();
    if num > 2 {
        let config = Config::new(&args);

        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);

        let content = fs::read_to_string(config.filename)
            .expect("Error while reading file");

        println!("With text:\n{}", content);
    } else {
        panic!("There are not enough arguments");
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config { query: args[1].clone(), filename: args[2].clone() }
    }
}
