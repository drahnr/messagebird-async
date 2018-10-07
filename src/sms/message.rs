use super::*;

/// Determines if the direction of the message
///
/// Mostly useful for filtering messages with `ListParamters`/`RequestMessageList`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename = "direction")]
pub enum Direction {
    #[serde(rename = "mt")]
    SendToMobile,
    #[serde(rename = "mo")]
    ReceivedFromMobile,
    #[serde(rename = "invalid")]
    Invalid,
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::SendToMobile => "mt",
            Direction::ReceivedFromMobile => "mo",
            _ => "invalid",
        }
    }
}

impl FromStr for Direction {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s).map_err(|_e| MessageBirdError::ParseError)
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

/// Determines the Gateway ID
///
/// Not very useful right now, recommended to not use unless you have explicit issues
/// i.e. some base stations in south eastern europe happily convert binary SMS to
/// textual SMS - because why not? In that case alternate routes might help to circumpass
/// the issues.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Gateway(pub u32);

impl Deref for Gateway {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Gateway {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s).map_err(|_e| MessageBirdError::ParseError)
    }
}

impl ToString for Gateway {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

// what is there to query
// to send, only originator,body and recipients are mandatory
// the rest is optional, but this would make it pretty annoying
// to use with almost every member being optional

/// BirdedMessage
///
/// A message as queried from the MessageBird API.
/// Refer to `SendableMessage` for an object which can be
/// sent.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    id: Identifier,
    href: Option<CallbackUrl>,
    direction: Direction,
    #[serde(rename = "type")]
    payload_type: PayloadType,
    originator: Option<Originator>,
    #[serde(rename = "body")]
    payload: Payload,
    reference: Option<String>,
    #[serde(flatten)]
    report_url: Option<CallbackUrl>,
    validity: Option<Duration>,
    gateway: Option<Gateway>,
    #[serde(rename = "typeDetails")]
    details: TypeDetails,
    #[serde(rename = "datacoding")]
    payload_encoding: PayloadEncoding,
    #[serde(rename = "mclass")]
    class: MessageClass,
    scheduled_datetime: Option<DateTime>,
    created_datetime: Option<DateTime>,
    recipients: Recipients,
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

    deser_roundtrip!(message_deser, Message, RAW);
}
