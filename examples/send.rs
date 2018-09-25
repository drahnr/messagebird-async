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

    info!("example: sending a message");
    let sendable = sms::send::QuerySend::builder()
        .payload(
            PayloadType::Sms,
            Payload::Text("fun".to_string()),
            PayloadEncoding::Auto,
        )
        .origin(AlphaNumeric("inbox".to_string()).into())
        .add_recipient(Msisdn::new(491637734321).unwrap().into())
        //.add_recipient(Recipient::new())
        .build();

    let accesskey = AccessKey::from_env()?;
    let fut = RequestSend::new(&sendable, &accesskey); //.and_then();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|_| ()))
}
