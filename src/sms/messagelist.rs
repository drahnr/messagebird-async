use super::*;

/// Helper for paging implementation
///
/// Currently not exposed/used. Requires some more thoughts on how to use this to implement a `Stream` of messages
///
/// TODO actually it would be awesome if this could be parsed back to `ListParameters`
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Links {
    first: Option<String>,
    previous: Option<String>,
    next: Option<String>,
    last: Option<String>,
}

/// BirdedMessage
///
/// A message as queried from the MessageBird API.
/// Refer to `SendableMessage` for an object which can be
/// sent.
///
/// Only meant for receiving
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MessageList {
    #[serde(rename = "offset")]
    offset: usize,
    #[serde(rename = "limit")]
    limit: usize,
    #[serde(rename = "count")]
    count: usize,
    #[serde(rename = "totalCount")]
    available: usize,
    #[serde(rename = "links")]
    links: Option<Links>,
    #[serde(rename = "items")]
    messages: Vec<Message>,
}

#[cfg(test)]
mod test {
    use super::*;

    static RAW_NONE: &str = r#"
{
    "offset": 15,
    "limit": 99,
    "count": 0,
    "totalCount": 0,
    "links": {
        "first": "https://rest.messagebird.com/messages/?offset=0&recipient=49333333&direction=mt",
        "previous": null,
        "next": null,
        "last": "https://rest.messagebird.com/messages/?offset=0&recipient=49333333&direction=mt"
    },
    "items": [
    ]
}
    "#;

    static RAW_ONE: &str = r#"
{
    "offset": 0,
    "limit": 20,
    "count": 1,
    "totalCount": 1,
    "links": {
        "first": "https://rest.messagebird.com/messages/?offset=0&recipient=49333333&direction=mt",
        "previous": null,
        "next": null,
        "last": "https://rest.messagebird.com/messages/?offset=0&recipient=49333333&direction=mt"
    },
    "items": [
        {
            "id": "4782ca7d6b6845ffa5c7ba5222a72e59",
            "href": "https://rest.messagebird.com/messages/4782ca7d6b6845ffa5c7ba5222a72e59",
            "direction": "mt",
            "type": "sms",
            "originator": "+497777777",
            "body": "fun",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-10-07T09:34:30+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-10-07T09:34:36+00:00"
                    }
                ]
            }
        }
    ]
}
    "#;

    static RAW_MULTI: &str = r#"
{
    "offset": 0,
    "limit": 20,
    "count": 7,
    "totalCount": 7,
    "links": {
        "first": "https://rest.messagebird.com/messages/?offset=0&recipient=49333333&direction=mt",
        "previous": null,
        "next": null,
        "last": "https://rest.messagebird.com/messages/?offset=0&recipient=49333333&direction=mt"
    },
    "items": [
        {
            "id": "4782ca7d6b6845ffa5c7ba5222a72e59",
            "href": "https://rest.messagebird.com/messages/4782ca7d6b6845ffa5c7ba5222a72e59",
            "direction": "mt",
            "type": "sms",
            "originator": "+497777777",
            "body": "fun",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-10-07T09:34:30+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-10-07T09:34:36+00:00"
                    }
                ]
            }
        },
        {
            "id": "fc9c4b7f3fe1472e95420b203baf8759",
            "href": "https://rest.messagebird.com/messages/fc9c4b7f3fe1472e95420b203baf8759",
            "direction": "mt",
            "type": "sms",
            "originator": "+497777777",
            "body": "fun",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-09-26T17:51:12+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-09-26T17:51:17+00:00"
                    }
                ]
            }
        },
        {
            "id": "159995298a6842488a874bd8ab1fa48c",
            "href": "https://rest.messagebird.com/messages/159995298a6842488a874bd8ab1fa48c",
            "direction": "mt",
            "type": "sms",
            "originator": "+497777777",
            "body": "fun",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-09-25T23:13:50+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-09-25T23:13:55+00:00"
                    }
                ]
            }
        },
        {
            "id": "8c67231711024c66ac0b5e41504d41ee",
            "href": "https://rest.messagebird.com/messages/8c67231711024c66ac0b5e41504d41ee",
            "direction": "mt",
            "type": "sms",
            "originator": "YourName",
            "body": "This",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-09-25T22:36:42+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-09-25T22:42:06+00:00"
                    }
                ]
            }
        },
        {
            "id": "a5473cb63a7b49e3a0a65f090e7861f6",
            "href": "https://rest.messagebird.com/messages/a5473cb63a7b49e3a0a65f090e7861f6",
            "direction": "mt",
            "type": "sms",
            "originator": "YourName",
            "body": "This is a test message",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-09-25T22:20:04+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-09-25T22:42:31+00:00"
                    }
                ]
            }
        },
        {
            "id": "31bb3bce954b4bdba0e8b5716ed4ac25",
            "href": "https://rest.messagebird.com/messages/31bb3bce954b4bdba0e8b5716ed4ac25",
            "direction": "mt",
            "type": "sms",
            "originator": "YourName",
            "body": "This is a test message",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-09-25T22:19:42+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-09-25T22:42:04+00:00"
                    }
                ]
            }
        },
        {
            "id": "aabcca29aa4c45e0963e47e713a073e3",
            "href": "https://rest.messagebird.com/messages/aabcca29aa4c45e0963e47e713a073e3",
            "direction": "mt",
            "type": "sms",
            "originator": "test",
            "body": "This is a test message",
            "reference": null,
            "validity": null,
            "gateway": 10,
            "typeDetails": {},
            "datacoding": "plain",
            "mclass": 1,
            "scheduledDatetime": null,
            "createdDatetime": "2018-09-03T00:41:26+00:00",
            "recipients": {
                "totalCount": 1,
                "totalSentCount": 1,
                "totalDeliveredCount": 1,
                "totalDeliveryFailedCount": 0,
                "items": [
                    {
                        "recipient": 49333333,
                        "originator": null,
                        "status": "delivered",
                        "statusDatetime": "2018-09-03T00:41:33+00:00"
                    }
                ]
            }
        }
    ]
}
    "#;

    deser_roundtrip!(message_list_none_deser, MessageList, RAW_NONE);
    // TODO origin will not be present and as such the final compare will fail
    //deser_roundtrip!(message_list_one_deser, MessageList, RAW_ONE);
    //deser_roundtrip!(message_list_multi_deser, MessageList, RAW_MULTI);

}
