extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate chrono;
extern crate url;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::time::Duration;
use url::Url;

use std::slice::Iter;
use std::vec::Vec;

#[macro_use]
pub mod macros;

mod errors;
use errors::*;

mod datetime;
use datetime::DateTime;
pub use datetime::*;

mod identifier;
pub use identifier::*;

mod callbackurl;
pub use callbackurl::*;

mod originator;
pub use originator::*;

mod recipient;
pub use recipient::*;

mod recipients;
pub use recipients::*;

mod payload;
pub use payload::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "direction")]
pub enum Direction {
    #[serde(rename = "mt")]
    SendToMobile,
    #[serde(rename = "mo")]
    ReceivedFromMobile,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway(pub u32);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum TypeDetail {
    #[serde(rename = "udh")]
    UserDataHeader(String),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename = "class")]
pub enum MessageClass {
    #[serde(rename = "0")]
    Class0,
    #[serde(rename = "1")]
    Class1,
    #[serde(rename = "2")]
    Class2,
    #[serde(rename = "3")]
    Class3,
}

use std::str::FromStr;

use std::collections::HashSet;

// what is there to query
// to send, only originator,body and recipients are mandatory
// the rest is optional, but this would make it pretty annoying
// to use with almost every member being optional
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    id: Identifier,
    href: Option<CallbackUrl>,
    direction: Direction,
    payload_type: PayloadType,
    originator: Originator,
    #[serde(rename = "body")]
    payload: Payload,
    reference: Option<String>,
    report_url: Option<CallbackUrl>,
    validity: Option<Duration>,
    gateway: Option<Gateway>,
    #[serde(rename = "typeDetails")]
    details: HashSet<TypeDetail>,
    #[serde(rename = "datacoding")]
    payload_encoding: PayloadEncoding,
    #[serde(rename = "mclass")]
    class: MessageClass,
    scheduled_datetime: Option<DateTime>,
    creation_datetime: Option<DateTime>,
    recipients: Recipients,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: Identifier::default(),
            href: None,
            direction: Direction::Invalid,
            payload_type: PayloadType::Sms,
            originator: Originator::Other(AlphaNumeric("invalid".to_string())),
            payload: Payload::Text("This is a default message".to_string()),
            reference: None,
            report_url: None,
            validity: None,
            gateway: None,
            details: HashSet::new(),
            payload_encoding: PayloadEncoding::Auto,
            class: MessageClass::Class0,
            scheduled_datetime: None,
            creation_datetime: None,
            recipients: Recipients::default(),
        }
    }
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder {
            message: Message::default(),
        }
    }
}

pub struct MessageBuilder {
    message: Message,
}

impl MessageBuilder {
    pub fn payload(
        mut self,
        payload_type: PayloadType,
        payload: Payload,
        payload_encoding: PayloadEncoding,
    ) -> Self {
        self.message.payload_type = payload_type;
        self.message.payload_encoding = payload_encoding;
        self.message.payload = payload;
        self
    }
    pub fn href(mut self, href: CallbackUrl) -> Self {
        self.message.href = Some(href);
        self
    }
    pub fn report_url(mut self, report_url: CallbackUrl) -> Self {
        self.message.report_url = Some(report_url); // FIXME
        self
    }
    pub fn origin(mut self, originator: Originator) -> Self {
        self.message.originator = originator;
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.message.direction = direction;
        self
    }
    pub fn recipient(mut self, recipient: Recipient) -> Self {
        self.message.recipients.add(recipient);
        self
    }
    pub fn identifier(mut self, identifier: Identifier) -> Self {
        self.message.id = identifier;
        self
    }
    pub fn build(self) -> Message {
        self.message
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    static RAW: &str = r#"
{
  "id":"e8077d803532c0b5937c639b60216938",
  "href":"https://rest.messagebird.com/messages/e8077d803532c0b5937c639b60216938",
  "direction":"mt",
  "type":"sms",
  "originator":"YourName",
  "body":"This is a test message",
  "reference":null,
  "validity":null,
  "gateway":null,
  "typeDetails":{},
  "datacoding":"plain",
  "mclass":1,
  "scheduledDatetime":null,
  "createdDatetime":"2016-05-03T14:26:57+00:00",
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
}
"#;

    #[test]
    fn roundtrip_serialize_deserialize() {
        let msg = Message::builder()
            .payload(
                PayloadType::Sms,
                Payload::Text("fun".to_string()),
                PayloadEncoding::Auto,
            )
            .origin(Originator::Other(AlphaNumeric("iamthesource".to_string())))
            .direction(Direction::SendToMobile)
            .recipient(Recipient::new())
            .build();

        let msg_str: String = serde_json::to_string(&msg).unwrap();
        println!("msg {}", msg_str);
    }

    #[test]
    fn roudtrip_deserialize_serialize() {
        let msg: Message = serde_json::from_str(RAW).unwrap();
        println!("msg {:?}", msg);

        let msg_str: String = serde_json::to_string(&msg).unwrap();
        println!("msg {}", msg_str);

        assert_eq!(RAW, msg_str)
    }
}
