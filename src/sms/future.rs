/// Query a list of messages
/// 
/// Still needs some work and the names are garbage

use super::*;

use futures::*;
use hyper;
use url;

#[derive(Debug,Clone)]
pub struct Query{
    uri : hyper::Uri
}

impl Query {
    pub fn parse(input : &str) -> Result<Query, MessageBirdError>{
        Ok(Self {
            uri : hyper::Uri::parse(input).map_err(|e| {
                    MessageBirdError::ParseError.context(e)?
        })
        })
    }

    pub fn builder() -> QueryBuilder {
        QueryBuilder::default()
    }

    pub fn as_str(&self) -> &str {
        self.uri.as_str()
    }
}

pub struct QueryBuilder {
    filter: String,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self {
            filter: String::new(),
        }
    }
}

impl QueryBuilder {
    fn with_originator(mut self, originator: Originator) -> Self {
        filter.append(originator.as_str());
        self
    }

    fn with_payload_type(mut self, payload_type: PayloadType) -> Self {
        filter.append(payload_type.as_str());
        self
    }

    fn with_direction(mut self, direction: Direction) -> Self {
        filter.append(payload_type.as_str());
        self
    }

    fn with_status(mut self, status: Status) -> Self {
        filter.append(status.as_str());
        self
    }

    fn contains_str(mut self, term: &str) -> Self {
        filter.append(String::from(term));
        self
    }

    fn between(mut self, start: DateTime, stop: DateTime) -> Self {
        self.from(start).until(stop)
    }

    fn from(mut self, start: DateTime) -> Self {
        unimplemented!()
    }

    fn until(mut self, stop: DateTime) -> Self {
        unimplemented!()
    }

    fn build(mut self) -> Query {
        debug!("query {}", &self.filter);
        let mut base = String::from("https://rest.messagebird.com/messages");
        if self.filter.len() > 0 {
        base.append("/");
        base.append(self.filter);
        }
        Query::parse(base.as_str()).expect("The builder shuld prevent parsing errors")
    }
}

pub struct Request {
    client: hyper::Client,
    query : Query,
    future : Box<Future<Item=(),Error=MessageBirdError>,
}

impl Request {
    pub fn new(query : Query) -> Self {
        let mut client = Client::new()

            // And then, if the request gets a response...
        client.get().and_then(|res| {
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
        });

        Self {
            client,
            query,
            future : 
        }
    }
}

impl Future for Request {
    type Item = ();
    type Error = MessageBirdError;
    fn poll(&mut self) -> Result<Async<Self::Item>,Self::Error> {
        future.poll()
    }
}
