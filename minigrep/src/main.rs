use minigrep::*;
use std::{env, error::Error, fs, process};

fn read_file(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    Ok(contents)
}
fn grep<'a>(contents: &'a String, config: &Config) -> Vec<&'a str> {
    if config.case_sensitive {
        return parse_file(contents, &config.query);
    }
    parse_file_case_insensitive(contents, &config.query)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args, true).unwrap_or_else(|err| {
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

    let result = grep(&contents, &config);
    if !result.is_empty() {
        println!("{:#?}", result);
    }
}
