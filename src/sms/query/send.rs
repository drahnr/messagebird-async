use super::*;

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
        let mut base = String::from("https://rest.messagebird.com/messages");
        let query = serde_url_params::to_string(self).unwrap();
        base.push_str("?");
        base.push_str(query.as_str());
        let uri: hyper::Uri = base.parse().unwrap();
        uri
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
        self.0.report_url = Some(report_url);
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn query_send() {
        let msisdns: Vec<Msisdn> =
            vec![Msisdn::new(123475).unwrap(), Msisdn::new(12345677).unwrap()];
        let url_params = QuerySend::builder()
            .payload(
                PayloadType::Sms,
                Payload::Text("fun".to_string()),
                PayloadEncoding::Auto,
            )
            .add_recipient(msisdns[0].into())
            .add_recipient(msisdns[1].into())
            .build();
        println!("send obj {:?}", url_params);
        let url_params_str = serde_url_params::to_string(&url_params).unwrap();
        println!("send params are \"{}\"", url_params_str);
    }
}
