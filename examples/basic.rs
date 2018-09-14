extern crate chrono;
extern crate messagebird_async;

use chrono::prelude::*;
use messagebird_async::sms;
use messagebird_async::sms::*;

fn main() {
    let uri = "https://rest.messagebird.com/messages".parse().unwrap();

    let q: Query<QueryMessages> = Query::<QueryMessages>::builder()
        .from(sms::DateTime::now())
        .until(sms::DateTime::now())
        .with_status(Status::Sent)
        .build();

    let q: Query<QueryMessages> = Query::<QueryMessages>::builder()
        .with_payload_type("")
        .with_direction("")
        .with_originator("198765432")
        .with_recipient("123456789")
        //.with_contact()
        .contains_term("fun").skip(5).limit(10).build();

    let fut = RequestMessages::new(q); //.and_then();
}
