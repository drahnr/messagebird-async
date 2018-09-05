use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;


#[derive(Debug)]
pub struct Identifier(String);

impl Default for Identifier {
    fn default() -> Self {
        Identifier("default".to_string())
    }
}

impl Identifier {
    pub fn new(raw: String) -> Self {
        Identifier(raw)
    }
}

impl FromStr for Identifier {
    type Err = MessageBirdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(String::from(s)))
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!("serialize has yet to be figured out")
        //serializer.serialize_str(self.0)
    }
}

struct IdentifierVisitor;

impl<'de> Visitor<'de> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date time formatted str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Identifier::from_str(value)
            .map_err(|e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IdentifierVisitor)
    }
}
