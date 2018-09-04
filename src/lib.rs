extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate datetime;
extern crate url;

use std::time::Duration;
use url::Url;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::slice::Iter;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "direction")]
pub enum Direction {
    #[serde(rename = "mt")]
    SendToMobile,
    #[serde(rename = "mo")]
    ReceivedFromMobile,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayloadType {
    Sms,
    Binary,
    Flash,
}

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize)]
pub struct TelephoneNumber {
    inner: String,
}

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize)]
pub struct AlphaNumeric {
    inner: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Originator {
    TelephoneNumber(TelephoneNumber),
    Other(AlphaNumeric),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway(pub u32);

// requires manual Serialize/Deserialize impl

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TypeDetail {
    #[serde(rename = "udh")]
    UserDataHeader(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "encoding")]
pub enum PayloadEncoding {
    Plain,
    Unicode,
    Auto,
}

#[derive(Debug, Serialize, Deserialize)]
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

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "msisdn")]
pub struct Msisdn {
    inner: u64,
}

impl Msisdn {
    pub fn new(raw: u64) -> Result<Self, u32> {
        Ok(Msisdn { inner: raw })
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
    status_datetime: DateTime, // FIXME TODO
}

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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    inner: String,
}

impl Identifier {
    pub fn new(raw: String) -> Self {
        Self { inner: raw }
    }
}

#[derive(Debug)]
pub struct CallbackUrl(Url);

impl Serialize for CallbackUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

use std::fmt;

struct CallbackUrlVisitor;

impl<'de> Visitor<'de> for CallbackUrlVisitor {
    type Value = CallbackUrl;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid url")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Url::parse(value)
            .map(|url| CallbackUrl(url))
            .map_err(|e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for CallbackUrl {
    fn deserialize<D>(deserializer: D) -> Result<CallbackUrl, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CallbackUrlVisitor)
    }
}






use std::str::FromStr;

#[derive(Debug)]
pub struct DateTime(datetime::LocalDateTime);

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!("serialize has yet to be figured out")
        //serializer.serialize_str(self.0)
    }
}

struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date time formatted str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        datetime::LocalDateTime::from_str(value)
            .map(|x| DateTime(x))
            .map_err(|e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}







// what is there to query
// to send, only originator,body and recipients are mandatory
// the rest is optional, but this would make it pretty annoying
// to use with almost every member being optional
#[derive(Debug, Serialize, Deserialize)]
struct Message<'a> {
    id: Identifier,
    href: CallbackUrl,
    direction: Direction,
    payload_type: PayloadType,
    originator: Originator,
    body: &'a [u8],
    reference: String,
    report_url: Option<CallbackUrl>,
    validity: Duration,
    gateway: Gateway,
    details: Vec<TypeDetail>,
    datacoding: PayloadEncoding,
    class: MessageClass,
    creation_point_of_time: Option<DateTime>,
    recipients: Recipients,
}

impl<'a> Message<'a> {
    pub fn new() -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn roudtrip_deserialize_serialize() {
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
        assert_eq!(2 + 2, 4);
    }

    fn roundtrip_serialize_deserialize() {
        unimplemented!("Yet to be implemented")
    }
}
