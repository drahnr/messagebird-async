# messagebird-async

MessageBird is a service for sending SMS at its core https://www.messagebird.com/

## Example

For a full example checkout `src/basic.rs` which will be inlined once the API becomes sufficiently stable.

## Features

Currently the planned features only include to send SMS and query SMS stati.


## RoadMap

 - [x] serde impl for all relevant datatypes
 - [x] roundtrip decode encode tests
 - [x] use [`crate eserde_plain`](https://docs.rs/serde_plain/0.3.0/serde_plain/) for `trait ToString` and `trait FromStr` for objects which are used in filters
 - [ ] future for SMS sending and retrieval based on [hyper.rs](https://hyper.rs)
 - [ ] send SMS example
 - [ ] future for querying SMS
 - [ ] useful examples using [tokio](https://tokio.rs) as executor
 - [ ] future for callback on SMS reception


## MessageBird APIs

 - [ ] [`SMS`](https://rest.messagebird.com/messages) (WIP)
 - [ ] [`Contacts`](https://rest.messagebird.com/contacts) (low prio)
 - [ ] [`MMS`](https://rest.messagebird.com/mms) (*)
 - [ ] [`Conversation`](https://developers.messagebird.com/docs/conversations) (*)
 - [ ] [`VoiceMessaging`](https://developers.messagebird.com/docs/voice-messaging) (*)
 - [ ] [`VoiceCalling`](https://developers.messagebird.com/docs/voice) (*)

\* = not planned, unless external contributors step up

## License

MIT or Apache-2.0

If neither of those works for you, feel free to request so via a github issue.

## Donations

Since I am not affiliated with MessageBird in any kind (other than being their customer)
I'd be delighted if you'd leave a tipp

[![LiberaPayButton](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/drahnr/donate)
