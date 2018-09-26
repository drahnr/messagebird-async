use super::*;

use serde::ser::{Serialize, Serializer};

use hyper;
use std::fmt;
use std::string::ToString;

/// TODO the name is misleading/obsolete, should be something with params 
pub trait Query {
    fn uri(&self) -> hyper::Uri;
    fn method(&self) -> hyper::Method {
        hyper::Method::GET
    }
}

// TODO
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Contact(u64);

impl Default for Contact {
    fn default() -> Self {
        Contact(0)
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contact({})", self.0)
    }
}

// TODO
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Group;

impl Default for Group {
    fn default() -> Self {
        Group
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no group")
    }
}

/// recpient for sending a message
///
/// Differs from the message format, such that it will serialize to a string
/// and can also be a group
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryRecipient {
    Group(Group),
    Msisdn(Msisdn),
}

impl From<Recipient> for QueryRecipient {
    fn from(_recipient: Recipient) -> Self {
        unimplemented!("TODO implement convenience conversion")
    }
}

impl ToString for QueryRecipient {
    fn to_string(&self) -> String {
        match self {
            QueryRecipient::Group(ref group) => group.to_string(),
            QueryRecipient::Msisdn(ref msisdn) => msisdn.to_string(),
        }
    }
}

impl FromStr for QueryRecipient {
    type Err = MessageBirdError;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        unimplemented!("TODO implement deserialize and all of the Group API")
    }
}

impl From<Msisdn> for QueryRecipient {
    fn from(msisdn: Msisdn) -> Self {
        QueryRecipient::Msisdn(msisdn)
    }
}

impl From<Group> for QueryRecipient {
    fn from(group: Group) -> Self {
        QueryRecipient::Group(group)
    }
}

impl Serialize for QueryRecipient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val_str = self.to_string();
        serializer.serialize_str(val_str.as_str())
    }
}

// only need one way for this one, ambiguity for recipients makes impl
// deserialize impossible without knowing all the existing group ids
// which would imply implementing the group id API
//
#[cfg(test)]
mod tests {

    #[derive(Debug, Serialize, Eq, PartialEq)]
    struct DummyQuery<T> {
        pub inner: T,
    }

    use super::*;
    #[test]
    fn recipient() {
        let recipient: QueryRecipient = Msisdn::new(123475).unwrap().into();

        let recipient = DummyQuery { inner: recipient };

        let recipient_str = serde_url_params::to_string(&recipient).unwrap();
        println!("recipient is {}", recipient_str);
    }

    #[test]
    fn recipient_vec() {
        let recipients: Vec<QueryRecipient> = vec![
            Msisdn::new(123475).unwrap().into(),
            Msisdn::new(777777777).unwrap().into(),
        ];

        let recipients = DummyQuery { inner: recipients };

        let recipients_str = serde_url_params::to_string(&recipients).unwrap();
        println!("recipients are \"{}\"", recipients_str);
    }

    #[test]
    fn recipient_optional_some() {
        let recipients: Option<QueryRecipient> = Some(Msisdn::new(123475).unwrap().into());

        let recipients = DummyQuery { inner: recipients };

        let recipients_str = serde_url_params::to_string(&recipients).unwrap();
        println!("recipient is Some(...) => \"{}\"", recipients_str);
    }

    #[test]
    fn recipient_optional_none() {
        let recipients: Option<QueryRecipient> = None;

        let recipients = DummyQuery { inner: recipients };

        let recipients_str = serde_url_params::to_string(&recipients).unwrap();
        println!("recipient is None => \"{}\"", recipients_str);
    }
}
