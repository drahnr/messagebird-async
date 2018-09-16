/// Query a list of messages
///
/// Still needs some work and the names are garbage
use super::*;

use hyper;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct QueryView;

#[derive(Debug, Clone)]
pub struct QueryMessages;

#[derive(Debug, Clone)]
pub struct QuerySend;

#[derive(Debug, Clone)]
pub struct Query<T> {
    uri: hyper::Uri,
    phantom: PhantomData<T>,
}

impl<T> Query<T> {
    pub fn parse(input: &str) -> Result<Query<T>, MessageBirdError> {
        Ok(Self {
            uri: input
                .parse::<hyper::Uri>()
                .map_err(|_e| MessageBirdError::ParseError)?,
            phantom: PhantomData,
        })
    }

    pub fn builder() -> QueryBuilder<T> {
        QueryBuilder::<T>::default()
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl<T> Deref for Query<T> {
    type Target = hyper::Uri;
    fn deref(&self) -> &Self::Target {
        &self.uri
    }
}

use std::fmt;

impl<T> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uri)
    }
}

pub struct QueryBuilder<T> {
    filter: String,
    phantom: PhantomData<T>,
}

impl<T> Default for QueryBuilder<T> {
    fn default() -> Self {
        Self {
            filter: String::new(),
            phantom: PhantomData,
        }
    }
}

impl QueryBuilder<QueryMessages> {
    pub fn originating_from(mut self, originator: Originator) -> Self {
        self.filter
            .push_str(&format!("&originator={}", originator.as_str()));
        self
    }

    pub fn with_payload_type(mut self, payload_type: PayloadType) -> Self {
        self.filter
            .push_str(&format!("&type={}", payload_type.as_str()));
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.filter
            .push_str(&format!("&direction={}", direction.as_str()));
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.filter
            .push_str(&format!("&status={}", status.as_str()));
        self
    }

    pub fn sent_to(mut self, msisdn: Msisdn) -> Self {
        self.filter.push_str(&format!("&recipient={}", *msisdn));
        self
    }

    pub fn count(mut self, upper_limit: u32) -> Self {
        self.filter.push_str(&format!("&limit={}", upper_limit));
        self
    }

    pub fn skip(mut self, skip: u32) -> Self {
        self.filter.push_str(&format!("&offset={}", skip));
        self
    }

    pub fn contains_term(mut self, term: &str) -> Self {
        self.filter.push_str(&format!("&searchterm={}", term));
        self
    }

    pub fn between(self, start: DateTime, stop: DateTime) -> Self {
        self.from(start).until(stop)
    }

    pub fn from(mut self, start: DateTime) -> Self {
        self.filter
            .push_str(&format!("&from={}", start.to_rfc3339()));
        self
    }

    pub fn until(mut self, stop: DateTime) -> Self {
        self.filter
            .push_str(&format!("&until={}", stop.to_rfc3339()));
        self
    }

    // TODO contact_id is missing, but that'd require having some kind of
    // abstraction over that in place

    pub fn build(self) -> Query<QueryMessages> {
        debug!("query {}", &self.filter);
        let mut base = String::from("https://rest.messagebird.com/messages");
        if self.filter.len() > 0 {
            base.push_str(&self.filter);
        }
        Query::<QueryMessages>::parse(base.as_str())
            .expect("The builder should prevent parsing errors")
    }
}

impl QueryBuilder<QueryView> {
    pub fn identifier(mut self, identifier: Identifier) -> Self {
        self.filter.push_str(identifier.as_str());
        self
    }
    // TODO contact_id is missing, but that'd require having some kind of
    // abstraction over that in place

    pub fn build(self) -> Query<QueryView> {
        debug!("query {}", &self.filter);
        let mut base = String::from("https://rest.messagebird.com/messages");
        if self.filter.len() > 0 {
            base.push_str("/");
            base.push_str(&self.filter);
        }
        Query::<QueryView>::parse(base.as_str()).expect("The builder should prevent parsing errors")
    }
}
