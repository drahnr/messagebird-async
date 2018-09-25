/// Query a list of messages
///
/// Still needs some work and the names are garbage
use super::*;

use futures::*;
use hyper;

use hyper_rustls;

use std::env;
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

impl AccessKey {
    pub fn from_env() -> Result<AccessKey, MessageBirdError> {
        let raw =
            env::var("MESSAGEBIRD_ACCESSKEY").map_err(|_e| MessageBirdError::AccessKeyError {
                msg: "env".to_string(),
            })?;
        AccessKey::from_str(raw.as_str())
    }
}

pub type RequestMessages = Request<query::list::QueryList, Vec<Message>>;
pub type RequestView = Request<query::view::QueryView, Message>;
pub type RequestSend = Request<query::send::QuerySend, Message>;

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
    debug!("request {:?}", request);
    let fut = client
            .request(request)
            .map_err(|_e: hyper::Error| MessageBirdError::RequestError)
            .and_then(|response: hyper::Response<hyper::Body>| {
                let status = response.status();
                debug!("rest status code: {}", status);

                // if status == hyper::StatusCode::OK {
                    futures::future::ok(response)
                // } else {
                //     futures::future::err(MessageBirdError::ServiceError {
                //         code: status.as_u16(),
                //         description : "TODO".to_string(),
                //         parameter : None,
                //     })
                // }
            })
            .and_then(|response: hyper::Response<hyper::Body>| {
                let body: hyper::Body = response.into_body();
                body.concat2().map_err(|_e| MessageBirdError::RequestError)
                // returns a hyper::Chunk!
            })
            // use the body after concatenation
            .and_then(|body| {
                debug!("response: {:?}", String::from_utf8(body.to_vec()).unwrap());
                // try to parse as json with serde_json
                let obj = serde_json::from_slice::<T>(&body).map_err(|_e| MessageBirdError::ParseError)?;
                Ok(obj)
            });
    fut
}

impl<Q, R> Request<Q, R>
where
    Q: Send + Query,
    R: 'static + Send + Sync + for<'de> serde::de::Deserialize<'de>,
{
    pub fn new(query: &Q, accesskey: &AccessKey) -> Self {
        let https = hyper_rustls::HttpsConnector::new(4);
        let mut client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        let mut request = hyper::Request::builder();
        request.uri(query.uri());
        request.method(query.method());
        request.header(
            hyper::header::AUTHORIZATION,
            format!("AccessKey {}", accesskey),
        );

        // XXX refactor needed - badly needed
        let request: hyper::Request<_> = if query.method() == hyper::Method::POST {
            request.header(
                hyper::header::CONTENT_TYPE,
                format!("application/x-www-form-urlencoded"),
            );
            if let Some(body) = query.uri().query() {
                let body = body.to_string();
                request.header(hyper::header::CONTENT_LENGTH, format!("{}", body.len()));
                request.body(body.into()).unwrap()
            } else {
                request.header(hyper::header::CONTENT_LENGTH, format!("{}", 0));
                request.body(hyper::Body::empty()).unwrap()
            }
        } else {
            request.header(hyper::header::CONTENT_LENGTH, format!("{}", 0));
            request.body(hyper::Body::empty()).unwrap()
        };

        let future = request_future_with_json_response::<R>(&mut client, request);
        // TODO avoid this boxing if possible
        let future = Box::new(future);
        Self {
            future,
            phantom: PhantomData,
        }
    }
}
