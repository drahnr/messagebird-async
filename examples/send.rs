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
use std::env;

fn main() -> Result<(), MessageBirdError> {
    env_logger::init();

    let sendable = SendableMessage::builder()
        .payload(
            PayloadType::Sms,
            Payload::Text("fun".to_string()),
            PayloadEncoding::Auto,
        )
        .origin(AlphaNumeric("iamthesource".to_string()).into())
        .add_recipient(Msisdn::new(123456789).unwrap().into())
        //.add_recipient(Recipient::new())
        .build();

    let accesskey = AccessKey::from_env()?;
    let fut = RequestSend::new(&sendable, &accesskey); //.and_then();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|_| ()))
}
