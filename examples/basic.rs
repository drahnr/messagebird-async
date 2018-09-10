use messagebird_async::sms::*;

fn main() {
        let uri = "https://rest.messagebird.com/messages".parse().unwrap();

        let q : Query = Query::builder()
        .between().add_filter().add_filter().build();

        let q : Query = Query::builder()
        .from(DateTime::now()).until().with_status(Status::Sent).build();

        let q : Query = Query::builder()
        .with_payload_type("")
        .with_direction("")
        .with_originator("198765432")
        .with_recipient("123456789")
        //.with_contact()
        .contains_term("fun").skip(5).limit(10).build();

        

        let fut = Request::new(q); //.and_then();
}
