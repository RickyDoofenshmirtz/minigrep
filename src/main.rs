use std::env;
use std::process;

use minigrap::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem in parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_name);

    if let Err(e) = minigrap::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
