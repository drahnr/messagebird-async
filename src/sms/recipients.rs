use super::*;

use super::recipient::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

// requires manual Serialize/Deserialize implementation
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Recipients {
    #[serde(rename = "totalCount")]
    total_count: u32,
    #[serde(rename = "totalSentCount")]
    total_sent: u32,
    #[serde(rename = "totalDeliveredCount")]
    total_delivered: u32,
    #[serde(rename = "totalDeliveryFailedCount")]
    total_delivery_failed: u32,
    #[serde(rename = "items")]
    items: Vec<Recipient>,
}

impl Default for Recipients {
    fn default() -> Self {
        Self {
            total_count: 0,
            total_sent: 0,
            total_delivered: 0,
            total_delivery_failed: 0,
            items: Vec::new(),
        }
    }
}

impl Recipients {
    pub fn count(&self) -> (u32, u32, u32) {
        (
            self.total_sent,
            self.total_delivered,
            self.total_delivery_failed,
        )
    }
    pub fn iter(&mut self) -> Iter<Recipient> {
        self.items.iter()
    }
    pub fn add(&mut self, recipient: Recipient) {
        self.items.push(recipient)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#"{
    "totalCount":1,
    "totalSentCount":1,
    "totalDeliveredCount":0,
    "totalDeliveryFailedCount":0,
    "items":[
      {
        "recipient": 31612345678,
        "status":"sent",
        "statusDatetime":"2016-05-03T14:26:57+00:00"
      }
    ]
}"#;

    deser_roundtrip!(recipients_deser, Recipients, RAW);
    serde_roundtrip!(recipients_serde, Recipients, Recipients::default());
}
