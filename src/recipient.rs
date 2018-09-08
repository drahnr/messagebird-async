use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename = "msisdn")]
pub struct Msisdn(u64);

/// Mobile Subscriber Integrated Services Digital Network Number
///
/// A worldwide unique phone number. This does not require a `+` or `00` prefix before the country code.
impl Msisdn {
    pub fn new(raw: u64) -> Result<Self, MessageBirdError> {
        if raw != 0 {
            Err(MessageBirdError::TypeError {
                msg: format!("Invalid phone number: {}", raw),
            })
        } else {
            Ok(Msisdn(raw))
        }
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
            msisdn: Msisdn(0),
            status: None,
            status_datetime: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static RAW: &str = r#"{
"recipient": 23747,
"status": "delivery_failed",
"statusDatetime" : "2016-05-03T14:26:57+00:00"
}"#;
    deser_roundtrip!(recipient_deser, Recipient, RAW);
    serde_roundtrip!(recipient_serde, Recipient, Recipient::new());
}
