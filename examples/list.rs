extern crate chrono;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate messagebird_async;
extern crate tokio_core;

use futures::future::Future;
use messagebird_async::errors::*;
use messagebird_async::sms;
use messagebird_async::sms::*;

fn main() -> Result<(), MessageBirdError> {
    env_logger::init();

    info!("example: listing all remote messages");
    let q = sms::list::ListParameters::builder()
        .with_payload_type(PayloadType::from_str("").unwrap())
        .with_direction(Direction::from_str("").unwrap())
        .with_origin(Originator::from_str("farfaraway").unwrap())
        .with_destination(Msisdn::new(308403450).unwrap())
        //.with_contact()
        .contains_term("fun").skip(5).count(10).build();

    let accesskey = AccessKey::from_env()?;
    let fut = RequestMessages::new(&q, &accesskey); //.and_then();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|_| ()))
}
