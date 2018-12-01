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
        .expect("SMS_RECIPIENT should contain the number without prefix");
    let msisdn: Msisdn = Msisdn::from_str(msisdn_str.as_str())
        .expect("SMS_RECIPIENT did not contain a valid number");

    info!("example: listing all remote messages");
    let q = sms::list::ListParameters::builder()
        //.with_payload_type(PayloadType::Sms) // curently the API has a bug which returns unexpected/undocumented JSON, so stay away from this for the time being
        .with_direction(Direction::SendToMobile)
        //.with_origin(Originator::from_str("inbox")?) // inbox is the default set by messagebird
        .with_destination(msisdn)
        //.with_contact() // unimplemented for the time being!
        //.contains_term("fun")
        .skip(1)
        .count(10)
        .build();

    let accesskey = AccessKey::from_env()?;
    let fut = RequestMessageList::new(&q, &accesskey);
    let fut = fut
        .and_then(|msgs: MessageList| {
            info!("{:?}", msgs);
            futures::future::ok(())
        }).map_err(|e| {
            debug!("err: {:?}", e);
            e
        });
    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(fut.map(|_| ()))
}
