/// Query a list of messages
///
/// Still needs some work and the names are garbage
use super::*;

use futures::*;
use hyper;

use hyper_rustls;

use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AccessKey(String);

impl Deref for AccessKey {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AccessKey {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO eval length
        Ok(AccessKey(s.to_string()))
    }
}

impl From<String> for AccessKey {
    fn from(s: String) -> Self {
        AccessKey(s)
    }
}

impl fmt::Display for AccessKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type RequestMessages = Request<QueryMessages, Vec<Message>>;
pub type RequestView = Request<QueryView, Message>;
pub type RequestSend = Request<QuerySend, ()>;

pub struct Request<T, R> {
    future: Box<Future<Item = R, Error = MessageBirdError>>,
    phantom: PhantomData<T>,
}

impl<T, R> Future for Request<T, R> {
    type Item = R;
    type Error = MessageBirdError;
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.future.poll()
    }
}

impl Request<QuerySend, ()> {
    pub fn new(sendable: &SendableMessage, accesskey: &AccessKey) -> Self {
        let query: Query<QuerySend> = Query::<QuerySend>::builder().build();

        let https = hyper_rustls::HttpsConnector::new(4);
        let client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        let mut request = hyper::Request::builder();
        request.uri(query.uri());
        request.method(hyper::Method::POST);
        request.header("Authorization", format!("AccessKey {}", accesskey));
        let json = serde_json::to_string(sendable).expect("Object is not serilializable");
        debug!("send SMS json: {}", json);
        let request = request.body(hyper::Body::from(json)).unwrap();

        // And then, if the request gets a response...
        let future = Box::new(
            client
                .request(request)
                .map_err(|_e| MessageBirdError::RequestError)
                .and_then(|res| {
                    let status = res.status();
                    debug!("send: status: {}", status);

                    if hyper::StatusCode::OK == status {
                        futures::future::ok(())
                    } else {
                        futures::future::err(MessageBirdError::ServiceError {
                            code: status.as_u16(),
                        })
                    }
                }),
        );

        Self {
            future,
            phantom: PhantomData,
        }
    }
}

fn request_future_with_json_response<T>(
    client: &mut hyper::Client<
        hyper_rustls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::Body,
    >,
    request: hyper::Request<hyper::Body>,
) -> impl Future<Item = T, Error = MessageBirdError>
where
    T: 'static + Sized + Send + Sync + for<'de> serde::de::Deserialize<'de>,
{
    let fut = client
            .request(request)
            .map_err(|_e: hyper::Error| MessageBirdError::RequestError)
            .and_then(|response: hyper::Response<hyper::Body>| {
                let status = response.status();
                debug!("rest status code: {}", status);

                if status == hyper::StatusCode::OK {
                    futures::future::ok(response)
                } else {
                    futures::future::err(MessageBirdError::ServiceError {
                        code: status.as_u16(),
                    })
                }
            })
            .and_then(|response: hyper::Response<hyper::Body>| {
                let body: hyper::Body = response.into_body();
                body.concat2().map_err(|_e| MessageBirdError::RequestError)
                // returns a hyper::Chunk!
            })
            // use the body after concatenation
            .and_then(|body| {
                // try to parse as json with serde_json
                let obj = serde_json::from_slice::<T>(&body).map_err(|_e| MessageBirdError::ParseError)?;
                Ok(obj)
            })  
            .map_err(|_e| MessageBirdError::ParseError);
    fut
}

impl Request<QueryView, Message> {
    pub fn new(query: &Query<QueryView>, accesskey: &AccessKey) -> Self {
        let https = hyper_rustls::HttpsConnector::new(4);
        let mut client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        let mut request = hyper::Request::builder();
        request.uri(query.uri());
        request.method(hyper::Method::GET);
        request.header("Authorization", format!("AccessKey {}", accesskey));
        let request: hyper::Request<_> = request.body(hyper::Body::empty()).unwrap();

        let future = request_future_with_json_response::<Message>(&mut client, request);
        // TODO avoid this boxing if possible
        let future = Box::new(future);
        Self {
            future,
            phantom: PhantomData,
        }
    }
}

impl Request<QueryMessages, Vec<Message>> {
    pub fn new(query: &Query<QueryMessages>, accesskey: &AccessKey) -> Self {
        let https = hyper_rustls::HttpsConnector::new(4);
        let mut client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        let mut request = hyper::Request::builder();
        request.uri(query.uri());
        request.method(hyper::Method::GET);
        request.header("Authorization", format!("AccessKey {}", accesskey));
        let request: hyper::Request<_> = request.body(hyper::Body::empty()).unwrap();

        let future = request_future_with_json_response::<Vec<Message>>(&mut client, request);
        // TODO avoid this boxing if possible
        let future = Box::new(future);
        Self {
            future,
            phantom: PhantomData,
        }
    }
}
