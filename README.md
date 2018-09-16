# messagebird-async

MessageBird is a service for sending SMS at its core https://www.messagebird.com/

## Example

For a full example checkout `src/basic.rs` which will be inlined once the API becomes sufficiently stable.

## Features

Currently the planned features only include to send SMS and query SMS stati.

There are no immediate plans to support the [`VoiceCalling`](https://developers.messagebird.com/docs/voice-messaging), [`VoiceMessaging`](https://developers.messagebird.com/docs/voice-messaging) or [`Conversation`](https://developers.messagebird.com/docs/conversations) API, if you feel like doing the work, I am happy to review PRs and integrate. [`Contacts`] API might be implemented since it can be used with the [`SMS Messaging`] API, but is a low prio.

## RoadMap

 - [x] serde impl for all relevant datatypes
 - [x] roundtrip decode encode tests
 - [x] use [`crate eserde_plain`](https://docs.rs/serde_plain/0.3.0/serde_plain/) for `trait ToString` and `trait FromStr` for objects which are used in filters
 - [ ] future for SMS sending and retrieval based on [hyper.rs](https://hyper.rs)
 - [ ] send SMS example
 - [ ] future for querying SMS
 - [ ] useful examples using [tokio](https://tokio.rs) as executor
 - [ ] future for callback on SMS reception
 - [ ] `Contacts` API (very low priority)

## License

MIT or Apache-2.0

If neither of those works for you, feel free to request so via a github issue.

## Affiliation/Sponsoring

Since I am not affiliated with MessageBird in any kind (other than being their customer)
I'd be delighted to if you'd leave a tipp

[![LiberaPayButton](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/drahnr/donate)
