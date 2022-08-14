use std::error::Error;
use std::process;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config).expect("Error while reading file");

}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            Ok(Config { query: args[1].clone(), filename: args[2].clone() })
        } else {
            return Err("There are not enough arguments")
        }
    }
}

// dyn Error => Dynamix error to not avoid to declare the type of error
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", content);
    Ok(())
}
