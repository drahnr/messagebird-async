use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;

use chrono;
use chrono::offset::{FixedOffset, Local, Offset};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DateTime(chrono::DateTime<FixedOffset>);

use std::time;
impl Default for DateTime {
    fn default() -> Self {
        let systime = time::SystemTime::now();
        let datetime_local = chrono::DateTime::<Local>::from(systime);
        let tz = chrono::offset::Utc.fix();
        let datetime_with_tz = datetime_local.with_timezone(&tz);
        DateTime(datetime_with_tz)
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_rfc3339().as_str())
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
        chrono::DateTime::parse_from_rfc3339(value)
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

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#""2016-05-03T14:26:57+00:00""#;
    deser_roundtrip!(datetime_deser, super::DateTime, RAW);
    serde_roundtrip!(datetime_serde, super::DateTime, super::DateTime::default());
}
