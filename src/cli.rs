use clap::{App, AppSettings, Arg};

use config::Config;
use utils::abort;

pub fn build_config_from_args() -> Config {
    let matches = App::new("hail")
        .version("0.1.0")
        .author("Sergey Potapov")
        .about("HTTP load testing tool")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("target")
                .value_name("TARGET_URL")
                .required(true)
                .help("Target URL")
                .index(1))
        .arg(
            Arg::with_name("concurrent")
                .long("concurrent")
                .short("c")
                .value_name("NUMBER")
                .default_value("10")
                .help("Number of concurrent requests"))
        .arg(
            Arg::with_name("requests")
                .long("requests")
                .short("r")
                .value_name("NUMBER")
                .default_value("1000")
                .help("Max number of requests to send"))
        .get_matches();
    Config::from_matches(&matches).unwrap_or_else(abort)
}
