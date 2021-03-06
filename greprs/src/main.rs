use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for - {}", config.query);
    println!("In file - {}", config.filename);
    
    let mut file = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading this file");

    println!("With text:\n{}", contents);
}

pub Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
