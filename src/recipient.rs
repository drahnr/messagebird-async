use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename = "msisdn")]
pub struct Msisdn(u64);

impl Msisdn {
    pub fn new(raw: u64) -> Result<Self, u32> {
        Ok(Msisdn(raw))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Unknown,
    Scheduled,
    Sent,
    Buffered,
    Delivered,
    Expired,
    DeliveryFailed,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Recipient {
    #[serde(rename = "recipient")]
    msisdn: Msisdn,
    #[serde(rename = "status")]
    status: Option<Status>,
    #[serde(rename = "statusDatetime")]
    status_datetime: Option<DateTime>,
}

impl Recipient {
    pub fn new() -> Self {
        Self {
            msisdn : Msisdn(0),
            status : None,
            status_datetime: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::macros::*;
    use super::*;
    serde_roundtrip!(recipient_serde, Recipient::new(), Recipient);
}