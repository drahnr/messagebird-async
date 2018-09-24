/// Query a list of messages
///
/// Still needs some work and the names are garbage
use super::*;

use hyper;

#[derive(Debug)]
pub struct QueryList {
    originator: Option<Originator>,
    recipient: Option<QueryRecipient>,
    direction: Option<Direction>,
    limit: Option<usize>,
    offset: Option<usize>,
    searchterms: Vec<String>,
    payload_type: Option<PayloadType>,
    contact_id: Option<Contact>,
    status: Option<Status>,
    start: Option<DateTime>,
    end: Option<DateTime>,
}

impl Default for QueryList {
    fn default() -> Self {
        QueryList {
            originator: None,
            recipient: None,
            direction: None,
            limit: None,
            offset: None,
            searchterms: vec![],
            payload_type: None,
            contact_id: None,
            status: None,
            start: None,
            end: None,
        }
    }
}

use std::fmt;
use std::string::String;

impl fmt::Display for QueryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xxx yyyy xxxxx")
    }
}

impl Query for QueryList {
    fn as_uri(&self) -> hyper::Uri {
        unimplemented!()
    }
}

pub struct Builder(QueryList);

impl Default for Builder {
    fn default() -> Self {
        Builder(QueryList::default())
    }
}

impl Builder {
    pub fn originating_from(mut self, originator: Originator) -> Self {
        self.0.originator = Some(originator);
        self
    }

    pub fn with_payload_type(mut self, payload_type: PayloadType) -> Self {
        self.0.payload_type = Some(payload_type);
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.0.direction = Some(direction);
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.0.status = Some(status);
        self
    }

    pub fn sent_to<T>(mut self, msisdn: T) -> Self
    where
        T: Into<QueryRecipient>,
    {
        self.0.recipient = Some(msisdn.into());
        self
    }

    pub fn count(mut self, upper_limit: u32) -> Self {
        self.0.limit = Some(upper_limit as usize);
        self
    }

    pub fn skip(mut self, skip: u32) -> Self {
        self.0.offset = Some(skip as usize);
        self
    }

    pub fn contains_term(mut self, term: &str) -> Self {
        self.0.searchterms.push(term.to_string());
        self
    }

    pub fn between(self, start: DateTime, stop: DateTime) -> Self {
        self.from(start).until(stop)
    }

    pub fn from(mut self, start: DateTime) -> Self {
        self.0.start = Some(start);
        self
    }

    pub fn until(mut self, stop: DateTime) -> Self {
        self.0.end = Some(stop);
        self
    }

    // TODO contact_id is missing, but that'd require having some kind of
    // abstraction over that in place

    pub fn build(self) -> QueryList {
        let mut base = String::from("https://rest.messagebird.com/messages");
        self.0
    }
}

impl QueryList {
    pub fn builder(self) -> Builder {
        let mut base = String::from("https://rest.messagebird.com/messages");
        Builder::default()
    }
}
