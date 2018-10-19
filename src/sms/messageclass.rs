use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;

use num::FromPrimitive;

/// Message class
///
/// TODO clarify the difference to `MessageType`
#[derive(Debug, Eq, PartialEq)]
pub enum MessageClass {
    /// displayed without being saved
    Class0 = 0,
    /// stored on the SIM as memory is available, 99% this is what you want
    Class1 = 1,
    /// message contains SIM card data, trigger response to the service center
    Class2 = 2,
    /// ...
    Class3 = 3,
}

impl Default for MessageClass {
    fn default() -> Self {
        MessageClass::Class1
    }
}

impl FromPrimitive for MessageClass {
    fn from_i64(n: i64) -> Option<Self> {
        if n < 0 {
            None
        } else {
            Self::from_u64(n as u64)
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0u64 => Some(MessageClass::Class0),
            1u64 => Some(MessageClass::Class1),
            2u64 => Some(MessageClass::Class2),
            3u64 => Some(MessageClass::Class3),
            _ => None,
        }
    }
}

impl MessageClass {
    pub fn as_u64(&self) -> u64 {
        match self {
            MessageClass::Class0 => 0,
            MessageClass::Class1 => 1,
            MessageClass::Class2 => 2,
            MessageClass::Class3 => 3,
        }
    }
}

impl Serialize for MessageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.as_u64())
    }
}

struct MessageClassVisitor;

impl<'de> Visitor<'de> for MessageClassVisitor {
    type Value = MessageClass;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid message class integer [0-3]")
    }

    // serde_json treats each number as u64?
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MessageClass::from_u64(value)
            .ok_or(de::Error::invalid_value(Unexpected::Unsigned(value), &self))
    }
}

impl<'de> Deserialize<'de> for MessageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(MessageClassVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#"
1
"#;

    deser_roundtrip!(messageclass_deser, MessageClass, RAW);
    serde_roundtrip!(messageclass_serde, MessageClass, MessageClass::default());
}
