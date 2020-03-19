use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;
use std::str::FromStr;

use chrono;
use chrono::offset::{FixedOffset, Local, Offset};
use std::ops::Deref;

use crate::errors::*;

/// Timestamp
///
/// A timestamp with a fixed offset.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct DateTime(chrono::DateTime<FixedOffset>);

impl Deref for DateTime {
    type Target = chrono::DateTime<FixedOffset>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl DateTime {
    pub fn now() -> Self {
        Self::default()
    }
}

impl FromStr for DateTime {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // workaround for messy messagebird API
        // see almost_rfc3339 test case
        let s_plus_recovered: String = s.replace(' ', "+");
        debug!("fmt datetime {} -> {}", s, s_plus_recovered);
        let s = s_plus_recovered.as_str();
        chrono::DateTime::parse_from_rfc3339(s)
            .or_else(|_err| {
                chrono::naive::NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S")
                    .and_then(|naive| Ok(chrono::DateTime::from_utc(naive, FixedOffset::west(0))))
            })
            .map(|datetime| DateTime(datetime))
            .map_err(|_e| MessageBirdError::FormatError {
                chunk: "Unexpected or invalid time format".to_string(),
            })
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
        Self::Value::from_str(value)
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
    mod rfc3339 {
        use super::*;
        static RAW: &str = r#""2016-05-03T14:26:57+00:00""#;
        deser_roundtrip!(datetime_deser, DateTime, RAW);
        serde_roundtrip!(datetime_serde, DateTime, DateTime::default());
    }
    mod almost_rfc3339 {
        use super::*;
        // XXX necessary, since the notification API is messed up and uses an improperly encoded rfc3339 timestamp
        // XXX which gets rided of it's plus sign
        static RAW: &str = r#"2016-05-03T14:26:57 00:00"#;
        #[test]
        fn deserialize() {
            let datetime = DateTime::from_str(RAW).expect("Failed to parse funny format");
            println!("Time parse from {} is {:?}", RAW, datetime);
        }
    }
    mod custom1 {
        use super::*;
        static RAW: &str = r#"20160503142657"#;
        #[test]
        fn deserialize() {
            let datetime = DateTime::from_str(RAW).expect("Failed to parse funny format");
            println!("Time parse from {} is {:?}", RAW, datetime);
        }
    }
}
