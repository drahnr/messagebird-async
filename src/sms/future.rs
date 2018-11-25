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

/// API Token Access
///
/// They can be managed under https://dashboard.messagebird.com/en/developers/access
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

/// Request object for a list of sent or in processing messages
pub type RequestMessageList = Request<parameter::list::ListParameters, MessageList>;

/// Request returning one individual message
pub type RequestView = Request<parameter::view::ViewParameters, Message>;

/// Request to send a message
pub type RequestSend = Request<parameter::send::SendParameters, Message>;

/// Generic API request object to messagebird REST API
/// Handles authorization and parses returned json into structures
///
/// Should not be used!
/// Use `RequestMessageList`,`RequestView`,`RequestSend` instead!
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

fn request_future_with_json_response<R>(
    client: &mut hyper::Client<
        hyper_rustls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::Body,
    >,
    request: hyper::Request<hyper::Body>,
) -> impl Future<Item = R, Error = MessageBirdError>
where
    R: 'static + Sized + Send + Sync + for<'de> serde::de::Deserialize<'de> + std::fmt::Debug,
{
    debug!("request {:?}", request);
    let fut = client
            .request(request)
            .map_err(|_e: hyper::Error| MessageBirdError::RequestError)
            .and_then(|response: hyper::Response<hyper::Body>| {
                let status = response.status();
                debug!("rest status code: {}", status);

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
                let status = response.status();
                let body: hyper::Body = response.into_body();
                body.concat2().map_err(|_e| MessageBirdError::RequestError).map(move |x|{(status,x)})
                // returns a hyper::Chunk!
            })
            // use the body after concatenation
            .and_then(|(status  ,body) : (_ , hyper::Chunk)| {

                debug!("response: {:?}", String::from_utf8(body.to_vec()).unwrap());
                match status {
                    hyper::StatusCode::OK => {
                        // try to parse as json with serde_json
                        match serde_json::from_slice::<R>(&body)
                            .map_err(|e| {
                                        debug!("Failed to parse response body: {:?}", e);
                                        MessageBirdError::ParseError
                                    })
                        {
                            Err(e) => futures::future::err(e),
                            Ok(x) => {
                                debug!("Parsed response {:?}", x);
                                futures::future::ok(x)
                            },
                        }
                    },
                    _ => {
                        match serde_json::from_slice::<ServiceErrors>(&body).map_err(|e| {
                            debug!("Failed to parse response body: {:?}", e);
                            MessageBirdError::ParseError}) {
                            Err(e) => futures::future::err(e),
                            Ok(x) => {
                                let x=x.into();
                                debug!("Parsed error response {:?}", x);
                                futures::future::err(MessageBirdError::ServiceError(x))
                            },
                        }
                    }
                }
            });
    fut
}

impl<P, R> Request<P, R>
where
    P: Send + Query,
    R: 'static + Send + Sync + for<'de> serde::de::Deserialize<'de> + std::fmt::Debug,
{
    pub fn new(parameters: &P, accesskey: &AccessKey) -> Self {
        let https = hyper_rustls::HttpsConnector::new(4);
        let mut client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);

        let mut request = hyper::Request::builder();
        request.uri(parameters.uri());
        request.method(parameters.method());
        request.header(
            hyper::header::AUTHORIZATION,
            format!("AccessKey {}", accesskey),
        );
        debug!("{:?}", request);

        // XXX refactor needed - badly needed
        let request: hyper::Request<_> = if parameters.method() == hyper::Method::POST {
            request.header(
                hyper::header::CONTENT_TYPE,
                format!("application/x-www-form-urlencoded"),
            );
            if let Some(body) = parameters.uri().query() {
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
