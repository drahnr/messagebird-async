use super::*;

use std::fmt;
use std::string::ToString;

/// QuerySend is an object that can be passed on to MessageBird API to trigger sending a SMS
#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct ViewParameters {
    #[serde(rename = "id")]
    identifier: Identifier,
}

impl ViewParameters {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: id.into(),
        }
    }

    fn id(&self) -> &Identifier {
        &self.identifier
    }
}

impl fmt::Display for ViewParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base = String::from("https://rest.messagebird.com/messages");
        //let query = serde_url_params::to_string(self).unwrap();
        let query = self.id().to_string();
        write!(f, "{}/{}", base, query)
    }
}

impl Query for ViewParameters {
    fn uri(&self) -> hyper::Uri {
        let uri: hyper::Uri = self.to_string().parse().unwrap();
        uri
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn query_view() {
        let url_params = ViewParameters::new(Identifier::new("someid".to_string()));
        println!("view obj {:?}", url_params);
        let url_params_str = serde_url_params::to_string(&url_params).unwrap();
        println!("view params are \"{}\"", url_params_str);
        assert_eq!(
            url_params.to_string(),
            "https://rest.messagebird.com/messages/someid".to_string()
        );
    }
}
