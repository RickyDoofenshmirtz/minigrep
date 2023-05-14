use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough argumnets!");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        return Ok(Config { query, file_name });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(config.file_name)?;
    println!("Contents:\n{}", content);
    Ok(())
}
