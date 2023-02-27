use std::fs;

pub fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");

    println!("Text is \n{contents}")
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
