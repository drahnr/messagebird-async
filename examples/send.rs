extern crate chrono;
#[macro_use]
extern crate log;
extern crate messagebird_async;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;

use chrono::prelude::*;
use messagebird_async::errors::*;
use messagebird_async::sms;
use messagebird_async::sms::*;
use futures::future::Future;

fn main() -> Result<(), MessageBirdError> {
    env_logger::init();

    let sendable = SendableMessage::builder()
        .payload(
            PayloadType::Sms,
            Payload::Text("fun".to_string()),
            PayloadEncoding::Auto,
        )
        .origin(AlphaNumeric("iamthesource".to_string()).into())
        .add_recipient(Recipient::new(123456789))
        //.add_recipient(Recipient::new())
        .build();

    let accesskey = AccessKey::from_str("034ujoensndf94")?;
    let fut = RequestSend::new(&sendable, &accesskey); //.and_then();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|_|()))
}
