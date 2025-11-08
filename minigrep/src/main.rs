use minigrep::parse_file;
use std::{env, error::Error, fs, process};

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Requires query and path");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        Ok(Config { query, path })
    }
}

fn read_file(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments : {err}");
        process::exit(1);
    });

    let contents = match read_file(&config) {
        Ok(content) => content,
        Err(e) => {
            println!("Reading Fille : {e}");
            process::exit(1);
        }
    };

    let result = parse_file(&contents, &config.query);
    if !result.is_empty() {
        println!("{:#?}", result);
    }
}
