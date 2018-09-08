use super::*;

use serde::de::{self, Deserialize, Deserializer, Expected, Unexpected, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct CallbackUrl(Url);

impl Serialize for CallbackUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

struct CallbackUrlVisitor;

impl<'de> Visitor<'de> for CallbackUrlVisitor {
    type Value = CallbackUrl;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid callback/href http url")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Url::parse(value)
            .map(|url| CallbackUrl(url))
            .map_err(|e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for CallbackUrl {
    fn deserialize<D>(deserializer: D) -> Result<CallbackUrl, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CallbackUrlVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static RAW: &str = r#"
"https://fun.with.https"
"#;

    deser_roundtrip!(url_deser, CallbackUrl, RAW);
    serde_roundtrip!(
        url_serde,
        CallbackUrl,
        CallbackUrl(Url::parse("https://ahoi.io/messagebird/tracker/cb").unwrap())
    );
}
