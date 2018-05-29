extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate tokio_io;
extern crate failure;
extern crate hyper_tls;

use tokio_io::{AsyncRead, AsyncWrite};
use std::io::{self, Write, Read, Cursor};
use futures::{Future, Stream, Async, Poll, task};
use hyper::{Client};
use hyper::header::UserAgent;
use tokio_core::reactor::Core;
// use hyper_tls::HttpsConnector;

// use hyper::header::ContentLength;
use hyper::server;

use failure::Error;

struct MockResp(Cursor<String>, bool);

impl MockResp {
    pub fn new(s: &String) -> Self {
        MockResp(Cursor::new(s.clone()), false)
    }
}

impl Read for MockResp {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.1 {
            self.0.read(buf)
        } else {
            Err(io::ErrorKind::WouldBlock.into())
        }
    }
}

impl Write for MockResp {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.1 = true;
        task::current().notify();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncRead for MockResp {

}

impl AsyncWrite for MockResp {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        Ok(Async::Ready(()))
    }

}

struct MockConn;

// const PHRASE: &'static str = "Hello, World!";

impl server::Service for MockConn {
    type Request = hyper::Uri;
    type Response = MockResp;
    type Error = io::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        Box::new(futures::future::ok(
            Self::Response::new(
                &"\
                 HTTP/1.1 200 OK\r\n\
                 Content-Length: 8\r\n\
                 \r\n\
                 Top kekking\
                 ".to_string()
            )
            // Self::Response::new()
            //     .with_header(ContentLength(PHRASE.len() as u64))
            //     .with_body(PHRASE)
        ))
    }
}

fn test() -> Result<(), Error> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        // .connector(HttpsConnector::new(4, &handle)?)
        .connector(MockConn)
        .build(&handle);

    let uri = "https://api.github.com/repos/CoproCodeForces-And-Friends/KFC-GitHub/issues".parse()?;
    //let mut req: Request = Request::new(Method::Get, uri);
    //req.headers_mut().set(UserAgent::new("curl/7.43.0"));
    let work = client.get(uri).and_then(|res| {
        //println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });
    let res = core.run(work)?;
    Ok(res)
}