//#![deny(warnings)]
//extern crate hyper;
#[macro_use]
extern crate serde_derive;
//extern crate serde;
//extern crate serde_json;
//extern crate primitives;

use primitives::OpaqueMetadata;
use primitives::{H256, blake2_256, hexdisplay::HexDisplay};
use srml_metadata::{
	DecodeDifferent, DecodeDifferentArray, FnEncode, RuntimeMetadata,
	ModuleMetadata, RuntimeMetadataV3,
	DefaultByteGetter, RuntimeMetadataPrefixed,
};
//#[cfg(feature = "std")]
//use parity_codec::codec::Decode;
#[cfg(feature = "std")]
use parity_codec::{Decode, Input};
//use parity_codec::{Encode, Output};
//use rstd::vec::Vec;

//use test_client::{runtime::{AccountId, Block, Hash, Index, Extrinsic, Transfer}, AccountKeyring::{self, *}};


use hyper::rt::{self, Future, Stream};
use hyper::{Client, Method, Request, Body};
use hyper::header::HeaderValue;

pub fn doit() {
    let url = "http://127.0.0.1:9933".parse().unwrap();

    let fut = fetch_json(url)
        // use the parsed vector
        .map(|ret| {
            //println!("result: {:#?}", ret.result);
            use parity_codec::Decode;
            use hex;
            let mut _hexstr = ret.result.clone();
            //println!("hex: {:?}", _hexstr);
            // cut 0x prefix: FIXME: expensive way to do it
            _hexstr.remove(0);
            _hexstr.remove(0);
            let _unhex = hex::decode(_hexstr)
                .expect("runtime metadata decoding hex failed");
            //println!("unhex: {:?}", _unhex);
            let mut _om = _unhex.as_slice();
            let _meta = RuntimeMetadataPrefixed::decode(&mut _om)
                .expect("runtime metadata decoding to RuntimeMetadataPrefixed failed.");
            //println!("decoded: {:?} ", _meta);
            let mut modules;
            match _meta.1 {
                RuntimeMetadata::V3(value) => {
                    match value.modules {
                        DecodeDifferent::Decoded(mods) => {
                            modules = mods;
                            println!("module0 {:?}", modules[0]);
                        },
                        _ => panic!("unsupported metadata"),
                    }
                },
                _ => panic!("unsupported metadata"),
            }
            println!("-------------------- modules ----------------");
            for module in modules {
                println!("module: {:?}", module.name);
                match module.name {
                    DecodeDifferent::Decoded(name) => {
                        match module.calls {
                            Some(DecodeDifferent::Decoded(calls)) => {
                                println!("calls: {:?}", calls);
                            },
                            _ => println!("ignoring"),
                        }
                        println!("storage: {:?}", module.storage)
                    },
                    _ => println!("ignoring"),
                }
            }
            //create new transaction
            
            //let xt = uxt(AccountKeyring::Alice, 1).encode();
        	//let h: H256 = blake2_256(&xt).into();
            
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });

    // Run the runtime with the future trying to fetch, parse and print json.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
    rt::run(fut);
}

fn fetch_json(url: hyper::Uri) -> impl Future<Item=JsonResponse, Error=FetchError> {
    let client = Client::new();

    let json = r#"{"method": "state_getMetadata", "params": null, "jsonrpc": "2.0", "id": 0}"#;
    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = url.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );

    client
        // Fetch the url...
        .request(req)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            let ret = serde_json::from_slice(&body)?;

            Ok(ret)
        })
        .from_err()
}

// Define a type so we can return multiple types of errors
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

#[derive(Deserialize, Debug)]
struct JsonResponse {
    jsonrpc: String,
    result: String,
}
#[test]
fn test_it(){
    doit();
}