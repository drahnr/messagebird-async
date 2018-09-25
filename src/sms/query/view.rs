use super::*;

/// QuerySend is an object that can be passed on to MessageBird API to trigger sending a SMS
#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct QueryView {
    #[serde(rename = "id")]
    identifier: Identifier,
}

impl QueryView {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: id.into(),
        }
    }
}

impl Query for QueryView {
    fn as_uri(&self) -> hyper::Uri {
        let mut base = String::from("https://rest.messagebird.com/messages");
        let query = serde_url_params::to_string(self).unwrap();
        base.push_str("?");
        base.push_str(query.as_str());
        let uri: hyper::Uri = base.parse().unwrap();
        uri
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn query_view() {
        let url_params = QueryView::new(Identifier::new("someid".to_string()));
        println!("view obj {:?}", url_params);
        let url_params_str = serde_url_params::to_string(&url_params).unwrap();
        println!("view params are \"{}\"", url_params_str);
    }
}
