/// Query a list of messages
/// 
/// Still needs some work and the names are garbage

use super::*;

use futures::*;
use hyper;

#[derive(Debug,Clone)]
pub struct Query {}

impl Query {
    pub fn builder() -> QueryBuilder {
        QueryBuilder::default()
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

    fn build(mut self) -> String {
        self.filter
    }
}

pub struct Request {
    client: hyper::Client,
    query : Query,
    future : Box<Future<Item=(),Error=MessageBirdError>,
}

impl Request {
    pub fn new(query : Query) {
        Self {
            client : Client::new(),
            query,
            future : Box::new(client.get())
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
