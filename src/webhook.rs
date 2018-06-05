extern crate failure;
extern crate futures;
extern crate hyper;
extern crate tokio;

use futures::future::ok;
use futures::Future;

use hyper::service::Service;
use hyper::{Body, Method, Request, Response, StatusCode};

struct WebhookService;

impl WebhookService {
    pub fn new() -> Self {
        WebhookService
    }
}

fn check_agent<T>(req: &Request<T>) -> bool {
    req.headers().get("User-Agent").map_or(false, |hv| {
        if let Ok(s) = hv.to_str() {
            s.split("/")
                .next()
                .map_or(false, |x| x == "GitHub-Hookshot")
        } else {
            false
        }
    })
}

impl Service for WebhookService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error>>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let mut res = Response::new(Body::empty());

        match (req.method(), req.uri().path()) {
            (&Method::POST, "/webhook") => {
                if check_agent(&req) {

                } else {
                    *res.status_mut() = StatusCode::FORBIDDEN;
                }
            }
            (&Method::GET, "/simple") => {
                *res.body_mut() = Body::from("{}");
            }
            _ => {
                *res.status_mut() = StatusCode::NOT_FOUND;
            }
        };
        Box::new(ok(res))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::Stream;
    use tokio::runtime::current_thread::Runtime;

    fn sts(s: hyper::Body) -> String {
        let body = s.map_err(|_| ())
            .fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            })
            .and_then(|v| String::from_utf8(v).map_err(|_| ()));
        body.wait().unwrap()
    }

    fn process_req(service: &mut WebhookService, req: Request<Body>) -> Response<Body> {
        let mut rt = Runtime::new().unwrap();
        let resp = service.call(req);
        rt.block_on(resp).unwrap()
    }

    #[test]
    fn test_example() {
        let req_good = Request::builder()
            .uri("/simple")
            .header("User-Agent", "GitHub-Hookshot/16abbec")
            .header("Content-Type", "application/json")
            .body(Body::from("{}"))
            .unwrap();

        let res_good = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("{}"))
            .unwrap();

        let mut service = WebhookService::new();
        let r = process_req(&mut service, req_good);
        assert_eq!(sts(res_good.into_body()), sts(r.into_body()));
    }

    #[test]
    fn test_rejects_bad_user_agent() {
        let req = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("User-Agent", "cool/agent")
            .header("Content-Type", "application/json")
            .body(Body::from("{}"))
            .unwrap();

        let mut service = WebhookService::new();
        let r = process_req(&mut service, req);
        assert_eq!(r.status(), StatusCode::FORBIDDEN);

        let req2 = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("User-Agent", "GitHub-Hookshot/16abbec")
            .header("Content-Type", "application/json")
            .body(Body::from("{}"))
            .unwrap();

        let r2 = process_req(&mut service, req2);
        assert_eq!(r2.status(), StatusCode::OK);
    }
}
