use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "msisdn")]
pub struct Msisdn(u64);

impl Msisdn {
    pub fn new(raw: u64) -> Result<Self, u32> {
        Ok(Msisdn(raw))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Scheduled,
    Sent,
    Buffered,
    Delivered,
    Expired,
    DeliveryFailed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Recipient {
    #[serde(rename = "recipient")]
    recipient: Msisdn,
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "statusDatetime")]
    status_datetime: DateTime,
}