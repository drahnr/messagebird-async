use super::*;
use std::collections::{hash_set::Iter, HashSet};
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Expected, MapAccess, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum TypeDetail {
    #[serde(rename = "udh")]
    UserDataHeader(String),
}

use std::hash::{Hash, Hasher};

impl Hash for TypeDetail {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TypeDetail::UserDataHeader(_) => 1.hash(state),
            _ => unreachable!("Mising implementation for struct variant"),
        }
    }
}

impl TypeDetail {
    pub fn try_from(src: (String, String)) -> Result<Self, MessageBirdError> {
        match src.0.as_str() {
            "udh" => Ok(TypeDetail::UserDataHeader(src.1)),
            x => Err(MessageBirdError::TypeError {
                msg: format!("Unknown TypeDetail \"{}\"", x),
            }),
        }
    }
    pub fn as_tuple(self) -> (String, String) {
        match self {
            TypeDetail::UserDataHeader(x) => (String::from("udh"), x),
            _ => unreachable!("xxxxxxxxxxxxxxxxxxxx"),
        }
    }
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
        let mut map = serializer.serialize_map(Some(self.inner.len()))?;
        for x in &self.inner {
            let (k, v) = x.clone().as_tuple();
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}

struct TypeDetailsVisitor;

impl<'de> Visitor<'de> for TypeDetailsVisitor {
    type Value = TypeDetails;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a TypeDetails Map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut set = TypeDetails::new();

        while let Some((key, value)) = access.next_entry()? {
            let p: (String, String) = (key, value);
            println!("TypeDetail key: {:?} value: {:?}", p.0, p.1);
            // TODO map error properly
            let td = TypeDetail::try_from(p).expect("Expected a valid type detail");
            set.add(td);
        }

        Ok(set)
    }
}

impl<'de> Deserialize<'de> for TypeDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TypeDetailsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static RAW_TYPE_DETAILS_EMPTY: &str = r#"
{}
"#;
    static RAW_TYPE_DETAILS_WITH_UDH: &str = r#"
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

    serde_roundtrip!(serde_typedetails_empty, TypeDetails, TypeDetails::new());
    deser_roundtrip!(deser_typedetails_empty, TypeDetails, RAW_TYPE_DETAILS_EMPTY);

    serde_roundtrip!(serde_typedetails_with_udh, TypeDetails, DETAILS.clone());
    deser_roundtrip!(
        deser_typedetails_with_udh,
        TypeDetails,
        RAW_TYPE_DETAILS_WITH_UDH
    );

    serde_roundtrip!(
        serde_typedetail_udh,
        TypeDetail,
        TypeDetail::UserDataHeader("some".to_string())
    );

}
