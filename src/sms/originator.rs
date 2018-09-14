use super::*;

use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};

use std::fmt;

use regex::Regex;

// TODO impl into() for Originator
// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TelephoneNumber(pub String);

impl TelephoneNumber {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for TelephoneNumber {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[1-9][0-9]+$").unwrap();
        }
        if RE.is_match(s) {
            Ok(TelephoneNumber(String::from(s)))
        } else {
            Err(MessageBirdError::FormatError {
                chunk: String::from(s),
            })
        }
    }
}

// requires manual Serialize/Deserialize impl
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AlphaNumeric(pub String);

impl AlphaNumeric {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for AlphaNumeric {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
        }
        if RE.is_match(s) {
            Ok(AlphaNumeric(String::from(s)))
        } else {
            Err(MessageBirdError::FormatError {
                chunk: String::from(s),
            })
        }
    }
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Originator {
    TelephoneNumber(TelephoneNumber),
    Other(AlphaNumeric),
}

impl FromStr for Originator {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TelephoneNumber::from_str(s)
            .and_then(|telephonenumber| Ok(Originator::TelephoneNumber(telephonenumber)))
            .or_else(|_e| {
                AlphaNumeric::from_str(s)
                    .and_then(|alphanumeric| Ok(Originator::Other(alphanumeric)))
            })
    }
}

impl Originator {
    pub fn as_str(&self) -> &str {
        match self {
            Originator::TelephoneNumber(ref telephonenumber) => telephonenumber.as_str(),
            Originator::Other(ref alphanumeric) => alphanumeric.as_str(),
        }
    }
}

impl From<TelephoneNumber> for Originator {
    fn from(telephonenumber: TelephoneNumber) -> Self {
        Originator::TelephoneNumber(telephonenumber)
    }
}

impl From<AlphaNumeric> for Originator {
    fn from(alphanumeric: AlphaNumeric) -> Self {
        Originator::Other(alphanumeric)
    }
}

struct OriginatorVisitor;

impl<'de> Visitor<'de> for OriginatorVisitor {
    type Value = Originator;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date time formatted str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Originator::from_str(value)
            .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for Originator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OriginatorVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#"
"123456789"
"#;

    deser_roundtrip!(originator_deser, Originator, RAW);
    serde_roundtrip!(
        originator_serde,
        Originator,
        TelephoneNumber::from_str("49123456789").unwrap().into()
    );
}
