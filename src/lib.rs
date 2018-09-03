extern crate datetime;
extern crate url;

use std::time::Duration;
use url::Url;

enum Direction {
    SendToMobile,
    ReceivedFromMobile,
}

enum PayloadType {
    Sms,
    Binary,
    Flash,
}

struct TelephoneNumber {
    inner: String,
}

struct AlphaNumeric {
    inner: String,
}

enum Originator {
    TelephoneNumber(TelephoneNumber),
    Other(AlphaNumeric),
}

enum Gateway {
    Gateway(u32),
}

enum TypeDetail {
    UserDataHeader(String),
}

enum PayloadEncoding {
    Plain,
    Unicode,
    Auto,
}

enum MessageClass {
    Class0,
    Class1,
    Class2,
    Class3,
}

struct Msisdn {}

enum Status {
    Scheduled,
    Sent,
    Buffered,
    Delivered,
    Expired,
    DeliveryFailed,
}

struct Recipient {
    recipient: Msisdn,
    status: Status,
    status_datetime: datetime::LocalDateTime,
}

struct Recipients {
    //    total_count : u32,
    total_sent: u32,
    total_delivered: u32,
    total_failed: u32,
    items: Vec<Recipient>,
}

struct Identifier {
    inner: String,
}

// what is there to query
// to send, only originator,body and recipients are mandatory
// the rest is optional, but this would make it pretty annoying
// to use with almost every member being optional
struct Message<'a> {
    id: Identifier,
    href: Url,
    direction: Direction,
    payload_type: PayloadType,
    originator: Originator,
    body: &'a [u8],
    reference: String,
    report_url: Option<Url>,
    validity: Duration,
    gateway: Gateway,
    details: Vec<TypeDetail>,
    datacoding: PayloadEncoding,
    class: MessageClass,
    creation_point_of_time: Option<datetime::LocalDateTime>,
    recipients: Recipients,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
