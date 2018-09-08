use super::*;
use std::collections::{hash_set::Iter, HashSet};
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TypeDetail {
    #[serde(rename = "udh")]
    UserDataHeader(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDetails {
    inner: HashSet<TypeDetail>,
}

impl TypeDetails {
    pub fn new() -> Self {
        Self {
            // TODO make sure the keys are unique, possibly implement Hash trait on TypeDetail manually
            inner: HashSet::new(),
        }
    }

    /// add a new type detail to the set of type details
    pub fn add(&mut self, td: TypeDetail) {
        self.inner.insert(td);
    }

    pub fn iter(&mut self) -> Iter<TypeDetail> {
        self.inner.iter()
    }
}

impl Serialize for TypeDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!("FIXME TypeDetails impl for serialize")
    }
}

struct TypeDetailsVisitor;

impl<'de> Visitor<'de> for TypeDetailsVisitor {
    type Value = TypeDetails;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date time formatted str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // Payload::from_str(value)
        //    .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
        unimplemented!("type detail deserialization fun")
    }
}

impl<'de> Deserialize<'de> for TypeDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TypeDetailsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static RAW_TYPE_DETAIL_EMPTY: &str = r#"
{}
"#;
    static RAW_TYPE_DETAIL_WITH_UDH: &str = r#"
{
    "udh" : "UserDataHeaderContent:)"
}
"#;

    lazy_static! {
        static ref DETAILS: TypeDetails = {
            let mut details = TypeDetails::new();
            details.add(TypeDetail::UserDataHeader("some".to_string()));
            details
        };
    }

    serde_roundtrip!(serde_typedetail_empty, TypeDetails, TypeDetails::new());
    serde_roundtrip!(serde_typedetail_with_udh, TypeDetails, DETAILS.clone());
    deser_roundtrip!(deser_typedetail_empty, TypeDetails, RAW_TYPE_DETAIL_EMPTY);
    deser_roundtrip!(
        deser_typedetail_with_udh,
        TypeDetails,
        RAW_TYPE_DETAIL_WITH_UDH
    );

}
