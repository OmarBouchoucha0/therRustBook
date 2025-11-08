pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn build(args: &Vec<String>, case_sensitive: bool) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Requires query and path");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        Ok(Config {
            query,
            path,
            case_sensitive,
        })
    }
}
pub fn parse_file<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut found: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found.push(line);
        }
    }
    found
}

pub fn parse_file_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut found: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            found.push(line);
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], parse_file(contents, query));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            parse_file_case_insensitive(contents, query)
        );
    }
}
