// use std::error::Error;












// 

pub mod config;
pub mod search;
pub mod commands;

use std::error::Error;
use crate::config::Config;
use crate::search::{search, search_case_insensitive, recursive_search};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.recursive {
        recursive_search(&config.query, &config.filename, config.case_sensitive)?;
    } else {
        let contents = std::fs::read_to_string(&config.filename)?;
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }
    }
    Ok(())
}
