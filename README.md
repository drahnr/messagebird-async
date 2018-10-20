# messagebird-async

[![Build Status](https://ci.spearow.io/api/v1/teams/main/pipelines/messagebird/jobs/master-validate/badge)](https://ci.spearow.io/teams/main/pipelines/messagebird) [![Crates.io](https://img.shields.io/crates/v/messagebird-async.svg)](https://crates.io/crates/messagebird-async) [![docs.rs](https://docs.rs/messagebird-async/badge.svg)](https://docs.rs/messagebird-async) [![License](https://img.shields.io/crates/l/messagebird-async.svg)](#license)

MessageBird is a service for sending SMS at its core https://www.messagebird.com/

## Example

### Send a SMS Message

Sending a sms to a specified target is implemented in `examples/send.rs`:

```sh
export MESSAGEBIRD_ACCESSKEY=abio8usad..dfahdk
export SMS_RECIPIENT=1234556
```

The `SMS_RECIPIENT` should NOT contain leading zeros nor the `+`. The countrycode is still necessary.

```sh
cargo run --example send
```

or copy & paste:

```rust
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
```

## Features

Currently the planned features only include to send SMS and query SMS stati.


## RoadMap

- [x] serde impl for all relevant datatypes
- [x] roundtrip decode encode tests
- [x] use [`crate serde_plain`](https://docs.rs/serde_plain/0.3.0/serde_plain/) for `trait ToString` and `trait FromStr` for objects which are used in query filters and payload www urlencode
- [x] future for SMS sending and retrieval based on [hyper.rs](https://hyper.rs)
- [x] send SMS example using [tokio](https://tokio.rs) as executor
- [x] future for listing SMS
- [x] listing SMS examples using [tokio](https://tokio.rs) as executor
- [x] write documentation
- [ ] future for callback on SMS reception
- [ ] callback example using [tokio](https://tokio.rs) as executor
- [ ] convert all service API return errors to typed errors

## MessageBird APIs

- [x] [`SMS`](https://rest.messagebird.com/messages) (see above)
- [ ] [`Contacts`](https://rest.messagebird.com/contacts) (low prio)
- [ ] [`MMS`](https://rest.messagebird.com/mms) (*)
- [ ] [`Conversation`](https://developers.messagebird.com/docs/conversations) (*)
- [ ] [`VoiceMessaging`](https://developers.messagebird.com/docs/voice-messaging) (*)
- [ ] [`VoiceCalling`](https://developers.messagebird.com/docs/voice) (*)

\* = not planned, unless external contributors step up

## License

MIT or Apache-2.0

If neither of those work for you, feel free to request so via a github issue.

## Donations

Since I am not affiliated with MessageBird in any kind (other than being their customer)
I'd be delighted if you'd leave a tipp to keep the automated send/receive routines going.

[![LiberaPayButton](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/drahnr/donate)
