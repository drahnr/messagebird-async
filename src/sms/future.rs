/// Query a list of messages
/// 
/// Still needs some work and the names are garbage

use super::*;

use futures::*;
use hyper;

use hyper_rustls;

use std::marker::PhantomData;

pub type RequestMessages = Request<QueryMessages>;
pub type RequestView = Request<QueryView>;
pub type RequestSend = Request<QuerySend>;

pub struct Request<T> {
    future : Box<Future<Item=(),Error=MessageBirdError>>,
    phantom : PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(query : Query<T>) -> Self {
        let https = hyper_rustls::HttpsConnector::new(4);
        let client : hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

            // And then, if the request gets a response...
        let future = Box::new(client.get(query.deref().clone()).and_then(|res| {
            println!("status: {}", res.status());

            // Concatenate the body stream into a single buffer...
            // This returns a new future, since we must stream body.
            res.into_body().concat2()
        })

        // And then, if reading the full body succeeds...
        .and_then(|body| {
            // The body is just bytes, but let's print a string...
            let s = ::std::str::from_utf8(&body)
                .expect("http bin sends utf-8 JSON");

            println!("body: {}", s);

            // and_then requires we return a new Future, and it turns
            // out that Result is a Future that is ready immediately.
            Ok(())
        })

        // Map any errors that might have happened...
        .map_err(|err| {
            println!("error: {}", err);
            MessageBirdError::ServiceError{code: 666}
        }));

        Self {
            future,
            phantom : PhantomData,
        }
    }
}

impl<T> Future for Request<T> {
    type Item = ();
    type Error = MessageBirdError;
    fn poll(&mut self) -> Result<Async<Self::Item>,Self::Error> {
        self.future.poll()
    }
}
