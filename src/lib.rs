
extern crate hyper;

use std::io::{self, Write};
use hyper::{Client, Method, Request, Body};
use hyper::header::HeaderValue;
use hyper::rt::{self, Future, Stream};
use serde_json::{Value, map::Map};



pub fn getMetadata() {
    rt::run(rt::lazy(|| {
        // This is main future that the runtime will execute.
        //
        // The `lazy` is because we don't want any of this executing *right now*,
        // but rather once the runtime has started up all its resources.
        //
        // This is where we will setup our HTTP client requests.
        let client = Client::new();

        let json = r#"{"method": "state_getMetadata", "params": null, "jsonrpc": "2.0", "id": 0}"#;
        let uri: hyper::Uri = "http://127.0.0.1:9933".parse().unwrap();
        let mut req = Request::new(Body::from(json));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );
        let post = client.request(req).and_then(|res| {
            println!("POST: {}", res.status());

            res.into_body().concat2()
        });

        //let body = post.wait().unwrap();

        
        post
            .map(|body| {
                println!("metadata result: {:?}", body);
                //let ret: &[u8] = serde_json::from_slice(&body).unwrap();

                //println!("GET: {:?}", got);
                
                //println!("extracted: {:?}", metadata["result"]);
                //Ok(body)
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    

    }));
}

// run local 'substrate --dev' node to run these tests
#[test]
fn test_getter() {
    getMetadata();

}