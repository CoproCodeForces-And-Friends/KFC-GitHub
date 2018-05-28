extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate failure;
extern crate hyper_tls;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::UserAgent;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let uri = "https://api.github.com/repos/CoproCodeForces-And-Friends/KFC-GitHub/issues".parse()?;
    let mut req: Request = Request::new(Method::Get, uri);
    req.headers_mut().set(UserAgent::new("curl/7.43.0"));
    let work = client.request(req).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });
    let res = core.run(work)?;
    Ok(res)
}