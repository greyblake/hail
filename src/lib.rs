extern crate clap;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod config;
mod utils;
mod stat;

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use futures::sync::mpsc;
use futures::Sink;
use std::time::Instant;

use clap::{App, AppSettings, Arg};

use std::cell::RefCell;

use config::Config;
use utils::{abort, humanize_duration};
use stat::Stat;

pub fn run() {
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


    let config = Config::from_matches(&matches).unwrap_or_else(abort);
    // println!("config = {:?}\n\n", config);
    attack(config);
}

fn attack(config: Config) {
    let stat_cell = RefCell::new(Stat::new());

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let uri = config.target.clone();

    let (tx, rx) = mpsc::channel(config.concurrent);
    let (res_tx, res_rx) = mpsc::channel(config.concurrent);

    let req_stream_f = rx.take(config.max_req+1).for_each(|_| {
        let tx = tx.clone();
        let res_tx = res_tx.clone();

        // Build a future for a new HTTP request
        let req_f = client.get(uri.clone())
            .then(move |req_res| {
                res_tx.send(req_res).wait().unwrap();

                // Add a new job to send a new request, when this one is done
                match tx.send(()).wait() {
                    Ok(_) => (),
                    Err(_) => ()
                };

                // Adapt this future to have type Future<Self::Item=(), Self::Error=()>, so it
                // can be passed to `handle.spawn()`
                let res: Result<(), ()> = Ok(());
                res
            });
        handle.spawn(req_f);

        // continue the stream
        Ok(())
    });


    let res_stream_f = res_rx.take(config.max_req).for_each(|result| {
        let response = result.unwrap_or_else(abort);
        let mut stat = stat_cell.borrow_mut();
        stat.add(response);
        Ok(())
    });


    let f = req_stream_f.join(res_stream_f);

    // Load channel with initial tasks
    for _ in 0..config.concurrent {
        tx.clone().send(()).wait().unwrap();
    }

    let start = Instant::now();
    core.run(f).expect("core.run() returned an error");
    let end = Instant::now();
    let time = end - start;

    let stat = stat_cell.borrow();

    let avg_response_time = (time * config.concurrent as u32) / (config.max_req as u32);

    println!("Requests sent: {}", config.max_req);
    println!("Total time: {}", humanize_duration(time));
    println!("Avg response time: {}", humanize_duration(avg_response_time));
    println!("OK rate: {:.2}%", stat.ok_rate() * 100.0);
    println!("Error rate: {:.2}%", stat.error_rate() * 100.0);
}
