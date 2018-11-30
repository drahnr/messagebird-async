use super::*;

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

/// Notification Url
///
/// An Url to be called on certain events directly from the MessageBird infrastructure
/// with some json.
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
            .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
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

    // TODO deserialization always appends the /
    // TODO so this is necessary until fixed properly
    static RAW: &str = r#"
"https://fun.with.https/"
"#;

    deser_roundtrip!(url_deser, CallbackUrl, RAW);
    serde_roundtrip!(
        url_serde,
        CallbackUrl,
        CallbackUrl(Url::parse("https://ahoi.io/messagebird/tracker/cb/").unwrap())
    );
}
