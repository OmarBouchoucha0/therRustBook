use std::{env, fs, process};
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

fn parse_file(contents: &str, query: &str) -> bool {
    for word in contents.split_whitespace() {
        if query == word {
            return true;
        }
    }
    false
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments : {err}");
        process::exit(1);
    });
    let contents = fs::read_to_string(config.path).expect("Coundt file the Path");

    if parse_file(&contents, &config.query) {
        println!("word found");
    } else {
        println!("word not found");
    }
}
