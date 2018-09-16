use super::*;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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
        serde_plain::from_str::<Self>(s).map_err(|_e| {
            MessageBirdError::ParseError
        })
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

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
        serde_plain::from_str::<Self>(s).map_err(|_e| {
            MessageBirdError::ParseError
        })
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
    originator: Originator,
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

/// SendableMessage is an object that can be passed on to MessageBird API to trigger sending a SMS
pub struct SendableMessage {
    // TODO
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
            details: TypeDetails::default(),
            payload_encoding: PayloadEncoding::Auto,
            class: MessageClass::Class0,
            scheduled_datetime: None,
            created_datetime: None,
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

    deser_roundtrip!(message_deser, Message, RAW);
    serde_roundtrip!(
        message_serde,
        Message,
        Message::builder()
            .payload(
                PayloadType::Sms,
                Payload::Text("fun".to_string()),
                PayloadEncoding::Auto
            )
            .origin(Originator::Other(AlphaNumeric("iamthesource".to_string())))
            .direction(Direction::SendToMobile)
            .recipient(Recipient::new())
            .recipient(Recipient::new())
            .build()
    );
}
