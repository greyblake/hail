extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use futures::sync::mpsc;
use futures::Sink;
use futures::sink::Send;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let client = Client::new(&handle);

    let uri: hyper::Uri = "http://127.0.0.1:4567/hello".parse().unwrap();

    let (tx, rx) = mpsc::channel(10);


    let work = rx.for_each(|res| {
        let tx = tx.clone();
        let req = client.get(uri.clone())
            .then(move |_| {
                println!("res = {:?}", res);
                tx.clone().send(res).wait();
                let r: Result<(), ()> = Ok(());
                r
            });
        handle.spawn(req);
        Ok(())
    });

    tx.clone().send(1).wait();
    tx.clone().send(2).wait();

    core.run(work).unwrap();
}
