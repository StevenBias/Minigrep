use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args.len();
    if num > 2 {
        let query = &args[1];
        let filename = &args[2];
        println!("Searching for {}", query);
        println!("In file {}", filename);
    } else {
        println!("There are not enough arguments");
    }
}
