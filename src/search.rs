pub struct Search {}

impl Search {
    pub fn case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        Search::search(content, &|line: &str| line.contains(query))
    }

    pub fn case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        Search::search(content, &|line: &str| {
            line.to_lowercase().contains(&query.to_lowercase())
        })
    }

    fn search<'a>(content: &'a str, predicate: &dyn Fn(&str) -> bool) -> Vec<&'a str> {
        content.lines().filter(|line| predicate(line)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_one_result_with_case_isensitive_search() {
        let query = "rUsT";
        let content = "
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], Search::case_insensitive(query, content));
    }

    #[test]
    fn should_find_no_result_with_case_sensitive_search() {
        let query = "rUsT";
        let content = "
Rust:
safe, fast, productive.
Pick three.";
        assert!(Search::case_sensitive(query, content).is_empty());
    }
}
