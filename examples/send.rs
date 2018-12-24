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

    let msisdn_str = std::env::var("SMS_RECIPIENT".to_string())
        .expect("Missing SMS_RECIPIENT environment variable");
    let msisdn: Msisdn = Msisdn::from_str(msisdn_str.as_str())?;

    info!("example: sending a message");
    let sendable = sms::send::SendParameters::builder()
        .payload(
            PayloadType::Sms,
            Payload::Text("fun".to_string()),
            PayloadEncoding::Auto,
        )
        .origin(AlphaNumeric("inbox".to_string()).into())
        .add_recipient(msisdn.into())
        //.add_recipient(Recipient::new())
        .build();

    let accesskey = AccessKey::from_env()?;
    let fut = RequestSend::new(&sendable, &accesskey);
    let fut = fut.and_then(|sent_msg: Message| {
        info!("{:?}", sent_msg);
        futures::future::ok(())
    });
    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|_| ()))
}
