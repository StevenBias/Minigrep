use std::process;
use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    minigrep::run(config).expect("Error while reading file");
}
