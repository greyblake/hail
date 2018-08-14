use clap::ArgMatches;
use hyper::Uri;

use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub target: Uri,
    pub concurrent: usize,
    pub max_req: u64,
}

impl Config {
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, String> {
        let target = fetch_opt(matches, "target")?;
        let concurrent = fetch_opt(matches, "concurrent")?;
        let max_req = fetch_opt(matches, "requests")?;

        let config = Self { target, concurrent, max_req };
        Ok(config)
    }
}

fn fetch_opt<T: FromStr>(matches: &ArgMatches, name: &str) -> Result<T, String> {
    matches.value_of(name).map_or(
        Err(format!("Option {} must be specified", name)),
        |given_value| {
            given_value.parse::<T>().or_else(|_| {
                Err(format!("Invalid value of {}: {}", name, given_value))
            })
        },
    )
}
