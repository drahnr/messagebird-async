use super::*;

use serde::ser::{Serialize, Serializer};

/// QuerySend is an object that can be passed on to MessageBird API to trigger sending a SMS
#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct QuerySend {
    // mandatory
    #[serde(rename = "originator")]
    originator: Originator,
    #[serde(rename = "body")]
    payload: Payload,
    // TODO this can actually be a mixture of Msisdns and group ids
    #[serde(rename = "recipients")]
    recipients: Vec<QueryRecipient>,
    // optionals, which should just not be there compared to : null
    #[serde(rename = "type")]
    payload_type: Option<PayloadType>,
    #[serde(flatten)]
    #[serde(rename = "reference")]
    reference: Option<String>,
    #[serde(flatten)]
    #[serde(rename = "reportUrl")]
    report_url: Option<CallbackUrl>,
    #[serde(flatten)]
    #[serde(rename = "validity")]
    validity: Option<Duration>,
    #[serde(flatten)]
    #[serde(rename = "gateway")]
    gateway: Option<Gateway>,
    #[serde(rename = "typeDetails")]
    details: Vec<TypeDetail>,
    #[serde(rename = "datacoding")]
    payload_encoding: Option<PayloadEncoding>,
    #[serde(rename = "mclass")]
    class: Option<MessageClass>,
    #[serde(flatten)]
    #[serde(rename = "scheduledDatetime")]
    scheduled_datetime: Option<DateTime>,
    // creation date is inferred by API usage
}

impl Default for QuerySend {
    fn default() -> Self {
        Self {
            payload_type: Some(PayloadType::Sms),
            originator: Originator::default(),
            payload: Payload::Text("This is a default message".to_string()),
            reference: None,
            report_url: None,
            validity: None,
            gateway: None,
            details: vec![],
            payload_encoding: Some(PayloadEncoding::Auto),
            class: Some(MessageClass::Class0),
            scheduled_datetime: None,
            recipients: vec![],
        }
    }
}

impl QuerySend {
    pub fn builder() -> Builder {
        Builder(QuerySend::default())
    }
}

impl Query for QuerySend {
    fn as_uri(&self) -> hyper::Uri {
        unimplemented!()
    }
}

pub struct Builder(QuerySend);

impl Builder {
    pub fn payload(
        mut self,
        payload_type: PayloadType,
        payload: Payload,
        payload_encoding: PayloadEncoding,
    ) -> Self {
        self.0.payload_type = Some(payload_type);
        self.0.payload_encoding = Some(payload_encoding);
        self.0.payload = payload;
        self
    }
    pub fn report_url(mut self, report_url: CallbackUrl) -> Self {
        self.0.report_url = Some(report_url); // FIXME
        self
    }
    pub fn origin(mut self, originator: Originator) -> Self {
        self.0.originator = originator;
        self
    }
    pub fn add_recipient(mut self, recipient: QueryRecipient) -> Self {
        self.0.recipients.push(recipient);
        self
    }
    pub fn build(self) -> QuerySend {
        self.0
    }
}

// only need one way for this one, ambiguity for recipients makes impl
// deserialize impossible without knowing all the existing group ids
// which would imply implementing the group id API
//
#[cfg(test)]
mod tests {

    #[derive(Debug, Serialize, Eq, PartialEq)]
    struct Frame<T> {
        pub inner: T,
    }

    use super::*;
    #[test]
    fn query_send_recipient() {
        let recipient: QueryRecipient = Msisdn::new(123475).unwrap().into();

        let recipient = Frame { inner: recipient };

        let recipient_str = serde_url_params::to_string(&recipient).unwrap();
        println!(" recipient is {}", recipient_str);
    }
    #[test]
    fn query_send_recipient_vec() {
        let recipients: Vec<QueryRecipient> = vec![
            Msisdn::new(123475).unwrap().into(),
            Msisdn::new(777777777).unwrap().into(),
        ];

        let recipients = Frame { inner: recipients };

        let recipients_str = serde_url_params::to_string(&recipients).unwrap();
        println!(" recipients are {}", recipients_str);
    }

    #[test]
    fn query_send_recipient_optional() {
        let recipients: Option<QueryRecipient> = Some(Msisdn::new(123475).unwrap().into());

        let recipients = Frame { inner: recipients };

        let recipients_str = serde_url_params::to_string(&recipients).unwrap();
        println!(" recipients are {}", recipients_str);
    }
    #[test]
    fn query_send_url_params() {
        let url_params = QuerySend::builder()
            .payload(
                PayloadType::Sms,
                Payload::Text("fun".to_string()),
                PayloadEncoding::Auto,
            )
            .add_recipient(Msisdn::new(123400001).unwrap().into())
            .add_recipient(Msisdn::new(567892213).unwrap().into())
            .build();
        let url_params_str = serde_url_params::to_string(&url_params).unwrap();
        println!(" params are {}", url_params_str);
    }

    //     serde_roundtrip!(
    //         query_send_serde,
    //         QuerySend,
    //         QuerySend::builder()
    //             .payload(
    //                 PayloadType::Sms,
    //                 Payload::Text("fun".to_string()),
    //                 PayloadEncoding::Auto
    //             )
    //             .origin(AlphaNumeric("iamthesource".to_string()).into())
    //             .add_recipient_msisdn(Msisdn::new(123400001).unwrap())
    //             .add_recipient_msisdn(Msisdn::new(567892213).unwrap())
    //             .build()
    //     );
}
