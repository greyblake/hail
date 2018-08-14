use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::{Core, Handle};
use futures::sync::mpsc;
use futures::Sink;
use std::time::Instant;

use std::cell::RefCell;

use config::Config;
use stat::Stat;
use report::Report;
use utils::abort;

pub fn attack(config: Config) -> Report {
    let stat_cell = RefCell::new(Stat::new());

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    // let client = Client::new(&handle);
    let client = build_client(&handle);

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
                tx.send(()).wait().ok();

                // Adapt this future to have type Future<Self::Item=(), Self::Error=()>, so it
                // can be passed to `handle.spawn()`
                Ok(()) as Result<(), ()>
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

    Report {
        concurrent: config.concurrent,
        total_time: time,
        ok_count: stat.ok_num,
        err_count: stat.err_num
    }
}

fn build_client(handle: &Handle) ->
    Client<::hyper_tls::HttpsConnector<::hyper::client::HttpConnector>, ::hyper::Body>
{
    let client = ::hyper::Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(4, handle).unwrap())
        .build(handle);

    client
}
