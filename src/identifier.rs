use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Identifier(String);

impl Default for Identifier {
    fn default() -> Self {
        Identifier("00000000000000000000000000000000".to_string())
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
        // XXX: taken from the example on the webpage
        const VALID_LENGTH: usize = 32;
        if s.len() != VALID_LENGTH {
            Err(MessageBirdError::TypeError {
                msg: format!(
                    "unexpected id length {}, expected {}",
                    s.len(),
                    VALID_LENGTH
                ),
            })
        } else {
            Ok(Self::new(String::from(s)))
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

struct IdentifierVisitor;

impl<'de> Visitor<'de> for IdentifierVisitor {
    type Value = Identifier;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid identifier str with 32 characters")
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

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#"
"01238dsfusd98ufe89hsdkncksadfkkr"
"#;

    deser_roundtrip!(identifier_deser, Identifier, RAW);
    serde_roundtrip!(identifier_serde, Identifier, Identifier::default());
}
