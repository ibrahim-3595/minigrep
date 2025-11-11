use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub recursive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let recursive = args.get(1).map(|s| s == "-r").unwrap_or(false);
        let (query, filename) = if recursive {
            (args[2].clone(), args[3].clone())
        } else {
            (args[1].clone(), args[2].clone())
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive, recursive })
    }
}