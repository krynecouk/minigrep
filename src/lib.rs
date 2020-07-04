mod search;

use search::Search;
use std::{env, error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        Search::case_sensitive(&config.query, &content)
    } else {
        Search::case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

/// Representation of parsed command line arguments.
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

/// Config methods and creators.
///
/// #Examples
///
/// ```
/// let args: Vec<String> = vec![
///     "minigrep".to_string(),
///     "test query".to_string(),
///     "~/dev/foo.txt".to_string(),
/// ];
/// let config = minigrep::Config::from(args.into_iter());
/// assert!(config.is_ok());
/// ```
impl Config {
    pub fn from<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next(); // name of program

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Missing query argument"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Missing filename argument"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config::new(&query, &filename, case_sensitive))
    }

    fn new(query: &str, filename: &str, case_sensitive: bool) -> Config {
        Config {
            query: query.to_string(),
            filename: filename.to_string(),
            case_sensitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_args_into_config() {
        let args: Vec<String> = vec![
            "minigrep".to_string(),
            "test query".to_string(),
            "~/dev/foo.txt".to_string(),
        ];
        let config = Config::from(args.into_iter());
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.query, "test query");
        assert_eq!(config.filename, "~/dev/foo.txt");
    }

    #[test]
    fn should_not_parse_args_if_query_is_missing() {
        let args: Vec<String> = vec!["minigrep".to_string()];
        let config = Config::from(args.into_iter());
        assert!(config.is_err());
        assert_eq!(config.err(), Some("Missing query argument"));
    }

    #[test]
    fn should_not_parse_args_if_filename_is_missing() {
        let args: Vec<String> = vec!["minigrep".to_string(), "test query".to_string()];
        let config = Config::from(args.into_iter());
        assert!(config.is_err());
        assert_eq!(config.err(), Some("Missing filename argument"));
    }
}
