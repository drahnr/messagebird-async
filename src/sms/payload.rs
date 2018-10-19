use super::*;

use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;

/// SMS encoding enum
///
/// Defines how to interpret the message encoding for text messages.
/// For binary SMS see `PayloadType`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "encoding")]
pub enum PayloadEncoding {
    Plain,
    Unicode,
    Auto,
}

/// SMS message type enum
///
/// Determines the type of the message payload
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PayloadType {
    /// regular text SMS, encoding defined by `PayloadEncoding`
    Sms,
    /// raw binary encoding of bytes, some providers are incapable of handling those with their base stations, be warned
    Binary,
    /// priority notification style SMS, there is no guarantee that this is stored on the phone
    Flash,
}

impl FromStr for PayloadType {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s).map_err(|_e| MessageBirdError::ParseError)
    }
}

impl ToString for PayloadType {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

impl PayloadType {
    pub fn as_str(&self) -> &str {
        match self {
            PayloadType::Sms => "sms",
            PayloadType::Binary => "binary",
            PayloadType::Flash => "flash",
        }
    }
}

/// Payload data
///
/// Enum representing both raw bytes/binary as well as text based sms messages.
///
/// Used for the sending direction.
///
/// `PayloadType` and `PayloadEncoding` are unrelated and used for querying.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Payload {
    Bytes(Vec<u8>),
    Text(String),
}

impl Default for Payload {
    fn default() -> Self {
        Payload::Text("default".to_string())
    }
}

// You can even choose to implement multiple traits, like Lower and UpperHex
impl fmt::LowerHex for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Payload::Bytes(ref bytes) => {
                for byte in bytes {
                    write!(f, "{:x} ", byte)?;
                }
            }
            Payload::Text(ref s) => {
                for byte in s.as_bytes() {
                    write!(f, "{:x} ", byte)?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Payload {
    type Err = MessageBirdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Payload::Text(String::from(s)))
    }
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Payload::Bytes(_) => {
                let data = format!("{:x}", self);
                serializer.serialize_str(data.as_str())
            }
            Payload::Text(ref s) => serializer.serialize_str(s.as_str()),
        }
    }
}

struct PayloadVisitor;

impl<'de> Visitor<'de> for PayloadVisitor {
    type Value = Payload;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid payload, either string or binary")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // TODO this actually requires context
        // TODO on how to parse the `value`
        // TODO without the type it is impossible to decide
        // TODO if i.e. 1234 is a Binary repr or a Text
        Payload::from_str(value)
            .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PayloadVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static RAW: &str = r#"
"16483910"
"#;
    deser_roundtrip!(payload_deser, Payload, RAW);
    serde_roundtrip!(payload_serde, Payload, Payload::default());
}
