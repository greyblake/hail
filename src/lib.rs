extern crate clap;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;

mod attack;
mod config;
mod utils;
mod stat;
mod report;
mod cli;

use utils::humanize_duration;
use report::Report;
use cli::build_config_from_args;

pub fn run() {
    let config = build_config_from_args();
    let report = attack::attack(config);
    print_report(&report);
}

fn print_report(report: &Report) {
    println!("Requests sent: {}", report.total_count());
    println!("Total time: {}", humanize_duration(report.total_time));
    println!("Avg response time: {}", humanize_duration(report.avg_time()));
    println!("OK rate: {:.2}%", report.ok_rate() * 100.0);
    println!("Error rate: {:.2}%", report.error_rate() * 100.0);
}
