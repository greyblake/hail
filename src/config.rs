use clap::ArgMatches;
use hyper::Uri;

use utils::fetch_opt;

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

        Ok(Self {
            target,
            concurrent,
            max_req
        })
    }
}
