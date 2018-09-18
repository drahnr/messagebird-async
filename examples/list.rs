extern crate chrono;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate messagebird_async;
extern crate tokio_core;

use chrono::prelude::*;
use futures::future::Future;
use messagebird_async::errors::*;
use messagebird_async::sms;
use messagebird_async::sms::*;

fn main() -> Result<(), MessageBirdError> {
    env_logger::init();

    // let q: Query<QueryMessages> = Query::<QueryMessages>::builder()
    //     .from(sms::DateTime::now())
    //     .until(sms::DateTime::now())
    //     .with_status(Status::Sent)
    //     .build();

    let q: Query<QueryMessages> = Query::<QueryMessages>::builder()
        .with_payload_type(PayloadType::from_str("").unwrap())
        .with_direction(Direction::from_str("").unwrap())
        .originating_from(Originator::from_str("198765432").unwrap())
        .sent_to(Msisdn::from_str("123456789").unwrap())
        //.with_contact()
        .contains_term("fun").skip(5).count(10).build();

    let accesskey = AccessKey::from_str("abc123")?;
    let fut = RequestMessages::new(&q, &accesskey); //.and_then();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|x| {
        log!("Result: {:?}", x);
        ()
    }))
}
