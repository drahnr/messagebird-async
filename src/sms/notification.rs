//! Notification via a registered callback from message bird
//!
//! Message bird calls a callback url as specified with a certain
//! set query parameters, virtual mobile number (VMN) and shortcode.
//! Both are represented here as structs which can be easily deserialized.
//!
//! messagebird documentation at
//! https://developers.messagebird.com/docs/sms-messaging#receive-a-message

// trait CallbackSpecifier {}

// pub struct ShortCode;
// impl CallbackSpecifier for ShortCode {}

// pub struct VirtualMobileNumber;
// impl CallbackSpecifier for VirtualMobileNumber {}

use super::*;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationQueryVMN {
    id: String,
    recipient: Msisdn,
    originator: Originator,
    #[serde(rename = "body")]
    payload: Payload,
    #[serde(rename = "createdDatetime")]
    created_datetime: DateTime, // RFC3339 format (Y-m-d\TH:i:sP)
}

impl FromStr for NotificationQueryVMN {
    type Err = MessageBirdError;
    fn from_str(query: &str) -> Result<Self, Self::Err> {
        serde_qs::from_str(query).map_err(|e| {
            debug!("{:?}", e);
            MessageBirdError::ParseError
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationQueryShort {
    mid: u64,
    shortcode: String,
    keyword: String,
    originator: Originator,
    operator: u64, //MCCMNC
    #[serde(rename = "message")]
    payload: Payload, // The body of the SMS message, including the (sub)keyword.
    receive_datetime: DateTime, // stamp format YmdHis
}

impl FromStr for NotificationQueryShort {
    type Err = MessageBirdError;
    fn from_str(query: &str) -> Result<Self, Self::Err> {
        serde_qs::from_str(query).map_err(|e| {
            debug!("{:?}", e);
            MessageBirdError::ParseError
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod vmn {
        use super::*;

        static RAW: &str = r#"http://your-own.url/script?id=e8077d803532c0b5937c639b60216938&recipient=31642500190&originator=31612345678&body=This+is+an+incoming+message&createdDatetime=2016-05-03T14:26:57+00:00"#;
        #[test]
        fn de() {
            let lh = NotificationQueryVMN {
                id: "e8077d803532c0b5937c639b60216938".to_string(),
                recipient: Msisdn::try_from(31642500190).unwrap(),
                originator: 31612345678.into(),
                payload: Payload::from_str("This is an incoming message").unwrap(),
                created_datetime: DateTime::from_str("2016-05-03T14:26:57+00:00").unwrap(),
            };

            let url = Url::parse(RAW).expect("Expected a valid url");
            // TODO the timestamp is modified due to the `+` sign which is replaced by a space character,
            // XXX as such parsing with rfc3339 fails
            let rh =
                serde_qs::from_str(url.query().unwrap()).expect("Failed to tokenize query string");
            assert_eq!(lh, rh);
        }
    }
    mod short {
        use super::*;
        use crate::sms::DateTime;

        static RAW: &str = r#"http://your-own.url/script?mid=123456789&shortcode=1008&keyword=MESSAGEBIRD&originator=31612345678&operator=20401&message=This+is+an+incoming+message&receive_datetime=20160503142657"#;
        #[test]
        fn de() {
            let lh = NotificationQueryShort {
                mid: 123456789,
                shortcode: "1008".to_string(),
                keyword: "MESSAGEBIRD".to_string(),
                originator: 31612345678.into(),
                operator: 20401,
                payload: Payload::from_str("This is an incoming message").unwrap(),
                receive_datetime: DateTime::from_str("2016-05-03T14:26:57+00:00").unwrap(),
            };

            let url = Url::parse(RAW).expect("Expected a valid url");
            let rh =
                serde_qs::from_str(url.query().unwrap()).expect("Failed to tokenize query string");
            assert_eq!(lh, rh);
        }
    }
}
