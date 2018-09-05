use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

// requires manual Serialize/Deserialize implementation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipients {
    total_count: u32,
    total_sent: u32,
    total_delivered: u32,
    total_failed: u32,
    items: Vec<Recipient>,
}

impl Default for Recipients {
    fn default() -> Self {
        Self {
            total_count: 0,
            total_sent: 0,
            total_delivered: 0,
            total_failed: 0,
            items: Vec::new(),
        }
    }
}

impl Recipients {
    pub fn count(&self) -> (u32, u32, u32) {
        (self.total_sent, self.total_delivered, self.total_failed)
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
    static RAW: &str = r#"
  "recipients":{
    "totalCount":1,
    "totalSentCount":1,
    "totalDeliveredCount":0,
    "totalDeliveryFailedCount":0,
    "items":[
      {
        "recipient":31612345678,
        "status":"sent",
        "statusDatetime":"2016-05-03T14:26:57+00:00"
      }
    ]
  }
"#;

    #[test]
    fn roudtrip_deserialize_serialize() {
        let recipients: Recipients = serde_json::from_str(RAW).unwrap();
        println!("recipients {:?}", recipients);

        let recipients_str: String = serde_json::to_string(&recipients).unwrap();
        println!("recipients {}", recipients_str);

        assert_eq!(RAW, recipients_str)
    }
}
