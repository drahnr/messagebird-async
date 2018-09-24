use super::*;

use hyper;
use serde::ser::{Serialize, Serializer};

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
        unimplemented!()
    }
}
