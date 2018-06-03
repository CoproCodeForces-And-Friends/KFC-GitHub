extern crate tokio;
extern crate hyper;
extern crate failure;
extern crate futures;

use futures::future::{ok, err, result};
use hyper::service::Service;
use hyper::{Body, Request, Response, StatusCode};
use tokio::runtime::current_thread::Runtime;

use hyper::rt::{Future, run};
use futures::Stream;

struct Test;

impl Test {
    pub fn new() -> Self {
        Test
    }
}

impl Service for Test {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error>>;
    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let res = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Hah haaay"))
            .unwrap();

        Box::new(ok(res))
    }
}

fn main() {
    println!("Lel");
}

#[cfg(test)]
mod test {
    use super::*;

    fn sts(s: hyper::Body) -> String {
        let body = s.map_err(|_| ()).fold(vec![], |mut acc, chunk| {
            acc.extend_from_slice(&chunk);
            Ok(acc)
        }).and_then(|v| String::from_utf8(v).map_err(|_| ()));
        body.wait().unwrap()
    }

    #[test]
    fn top_test() {
        let mut server = Test::new();
        let req = Request::builder()
            .uri("http://fominok.ru")
            .header("User-Agent", "my-agent/1.0")
            .body(Body::from(""))
            .unwrap();

        let good = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Hah haaay"))
            .unwrap();


        let mut rt = Runtime::new().unwrap();
        let resp = server.call(req);
        let r = rt.block_on(resp).unwrap();
        assert_eq!(sts(good.into_body()), sts(r.into_body()));
    }
}
