extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate messagebird_async as messagebird;
extern crate tokio_core;

use futures::{future, Future};

use hyper::client::HttpConnector;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode};

static NOTFOUND: &[u8] = b"Not Found";

fn incoming(
    req: Request<Body>,
    _client: &Client<HttpConnector>,
) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    println!("incoming!");
    let method = req.method();
    let uri = req.uri();
    match (method, uri.path(), uri.query()) {
        (&Method::GET, "/vmn", Some(query)) => {
            let x = query.parse::<messagebird::sms::NotificationQueryVMN>().expect("Failed to parse");
            println!("notfied of vmn sms {:?}", x);
            let body = format!("notfied of shortcode sms {:?}", x);
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(hyper::header::CONTENT_LENGTH, format!("{}", body.len()))
                .body(body.into())
                .unwrap();
            Box::new(future::ok(response))
        }
        (&Method::GET, "/short", Some(query)) | (&Method::GET, "/shortcode", Some(query)) => {
            let x = query.parse::<messagebird::sms::NotificationQueryShort>().expect("Failed to parse");
            println!("notfied of shortcode sms {:?}", x);
            let body = format!("notfied of shortcode sms {:?}", x);
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(hyper::header::CONTENT_LENGTH, format!("{}", body.len()))
                .body(body.into())
                .unwrap();
            Box::new(future::ok(response))
        }
        _ => {
            // Return 404 not found response.
            let body = Body::from(NOTFOUND);
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap();
            Box::new(future::ok(response))
        }
    }
}

fn main() {
    env_logger::init();

    let addr = "127.0.0.1:8181".parse().unwrap();

    hyper::rt::run(future::lazy(move || {
        let client = Client::new();

        let service = move || {
            let client = client.clone();
            service_fn(move |req| incoming(req, &client))
        };

        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }));
}
