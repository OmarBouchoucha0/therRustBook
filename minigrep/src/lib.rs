pub fn parse_file<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut found: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
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
}
