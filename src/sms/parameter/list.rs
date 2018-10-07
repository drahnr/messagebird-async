/// Query a list of messages
///
/// Still needs some work and the names are garbage
use super::*;

use hyper;

/// Query Parameters
///
/// Filters the results of the query on the backend service side.
#[derive(Debug, Serialize)]
pub struct ListParameters {
    // #[serde(flatten)]
    originator: Option<Originator>,
    recipient: Option<QueryRecipient>,
    // #[serde(flatten)]
    direction: Option<Direction>,
    #[serde(flatten)]
    limit: Option<usize>,
    #[serde(flatten)]
    offset: Option<usize>,
    #[serde(rename = "searchterm")]
    searchterms: Vec<String>,
    // #[serde(flatten)]
    #[serde(rename = "type")]
    payload_type: Option<PayloadType>,
    // #[serde(flatten)]
    contact_id: Option<Contact>,
    // #[serde(flatten)]
    status: Option<Status>,
    #[serde(flatten)]
    start: Option<DateTime>,
    #[serde(flatten)]
    end: Option<DateTime>,
}

impl Default for ListParameters {
    fn default() -> Self {
        ListParameters {
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

impl fmt::Display for ListParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base = String::from("https://rest.messagebird.com/messages");
        let query = serde_url_params::to_string(self).unwrap();
        write!(f, "{}?{}", base, query)
    }
}

impl Query for ListParameters {
    fn uri(&self) -> hyper::Uri {
        let uri: hyper::Uri = self
            .to_string()
            .parse()
            .expect("Failed to parse list query object to hyper::Uri");
        uri
    }
}

impl ListParameters {
    /// Construct the `ListParameters` using the builder pattern
    pub fn builder() -> Builder {
        Builder::default()
    }
}

pub struct Builder(ListParameters);

impl Default for Builder {
    fn default() -> Self {
        Builder(ListParameters::default())
    }
}

impl Builder {
    /// Filters for the origin of the message
    ///
    /// May be either a phone number, contact (unimplemented as of now) or a string.
    /// Messages which are sent via the API and do not have an origin defined will have `"inbox"`
    /// be the default originator.
    pub fn with_origin(mut self, originator: Originator) -> Self {
        self.0.originator = Some(originator);
        self
    }

    /// Filters for the payload type, either being `SMS` or `Binary`
    pub fn with_payload_type(mut self, payload_type: PayloadType) -> Self {
        self.0.payload_type = Some(payload_type);
        self
    }

    /// Filters for the direction
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.0.direction = Some(direction);
        self
    }

    /// Filters for the status of the messages being listed
    pub fn with_status(mut self, status: Status) -> Self {
        self.0.status = Some(status);
        self
    }

    /// Counterpart to `with_origin`
    pub fn with_destination<T>(mut self, msisdn: T) -> Self
    where
        T: Into<QueryRecipient>,
    {
        self.0.recipient = Some(msisdn.into());
        self
    }

    /// The number of messages to skip before listing.
    /// Allows it to be used as paginating.
    pub fn skip(mut self, skip: u32) -> Self {
        self.0.offset = Some(skip as usize);
        self
    }

    /// Limits the number of returned messages
    pub fn count(mut self, upper_limit: u32) -> Self {
        self.0.limit = Some(upper_limit as usize);
        self
    }

    /// Validate the message contained a fixed term
    pub fn contains_term(mut self, term: &str) -> Self {
        self.0.searchterms.push(term.to_string());
        self
    }

    /// Filters for messages that were sent in a certain range
    pub fn between(self, start: DateTime, stop: DateTime) -> Self {
        self.from(start).until(stop)
    }

    /// Defines the timspan since when the message was sent
    /// TODO rename, too amibiguous
    pub fn from(mut self, start: DateTime) -> Self {
        self.0.start = Some(start);
        self
    }

    /// Until what timestamp the messages we are interested in are filtered
    pub fn until(mut self, stop: DateTime) -> Self {
        self.0.end = Some(stop);
        self
    }

    // TODO contact_id is missing, but that'd require having some kind of
    // abstraction over that in place
    /// Finalizes and returns the `ListParameters`
    pub fn build(self) -> ListParameters {
        self.0
    }
}

// only need one way for this one, ambiguity for recipients makes impl
// deserialize impossible without knowing all the existing group ids
// which would imply implementing the group id API
#[cfg(test)]
mod tests {

    // helper construct for testing
    #[derive(Debug, Serialize, Eq, PartialEq)]
    struct Frame<T> {
        pub inner: T,
    }

    use super::*;
    #[test]
    fn query_list() {
        let msisdns: Vec<Msisdn> =
            vec![Msisdn::new(123475).unwrap(), Msisdn::new(12345677).unwrap()];
        let url_params = ListParameters::builder()
            .contains_term("fun")
            .with_destination(msisdns[0])
            .build();
        println!("list obj {:?}", url_params);

        let url_params_str = serde_url_params::to_string(&url_params).unwrap();
        println!("list params are \"{}\"", url_params_str);
        assert_eq!(
            url_params.to_string(),
            "https://rest.messagebird.com/messages?recipient=123475&searchterm=fun".to_string()
        );
    }
}
