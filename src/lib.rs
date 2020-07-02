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

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config::new(&args[1], &args[2], case_sensitive))
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
            "_".to_string(),
            "test query".to_string(),
            "~/dev/foo.txt".to_string(),
        ];
        let config = Config::from(&args);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.query, "test query");
        assert_eq!(config.filename, "~/dev/foo.txt");
    }

    #[test]
    fn should_not_parse_args_into_config_if_wrong_number_of_arguments() {
        let args: Vec<String> = vec![
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
            "_".to_string(),
        ];
        let config = Config::from(&args);
        assert!(config.is_err());
    }

    #[test]
    fn should_not_parse_args_into_config_if_no_arguments() {
        let args: Vec<String> = vec![];
        let config = Config::from(&args);
        assert!(config.is_err());
    }
}
