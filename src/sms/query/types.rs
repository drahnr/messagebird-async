use super::*;

use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;

use std::string::ToString;

pub trait Query {
    fn as_uri(&self) -> hyper::Uri;
}

// TODO
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Contact(u64);

impl Default for Contact {
    fn default() -> Self {
        Contact(0)
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
impl ToString for Group {
    fn to_string(&self) -> String {
        "no group".to_string()
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
