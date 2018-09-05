use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

use chrono;
use chrono::prelude::*;



#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DateTime(chrono::DateTime<Utc>);

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!("serialize has yet to be figured out")
        //serializer.serialize_str(self.0)
    }
}

struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date time formatted str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        chrono::DateTime::<Utc>::from_str(value)
            .map(|x| DateTime(x))
            .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}