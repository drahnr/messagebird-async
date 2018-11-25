use super::*;

use num::{FromPrimitive, ToPrimitive};
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

/// error codes
///
/// Error codes as returned as part of a response from the service in the payload.
/// These are NOT http status codes.
#[derive(Primitive, Debug, PartialEq, Eq, Clone)]
pub enum ServiceErrorCode {
    RequestNotAllowed = 2,
    MissingParameters = 9,
    InvalidParameters = 10,
    NotFound = 20,
    BadRequest = 21,
    NotEnoughBalance = 25,
    EndpointNotFound = 98,
    InternalError = 99,
}

/// Visitor for parsing the ServiceErrorCode from integers
struct ErrorCodeVisitor;

impl<'de> Visitor<'de> for ErrorCodeVisitor {
    type Value = ServiceErrorCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid ServiceErrorCode")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ServiceErrorCode::from_u64(value)
            .ok_or(de::Error::invalid_value(Unexpected::Unsigned(value), &self))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ServiceErrorCode::from_i64(value)
            .ok_or(de::Error::invalid_value(Unexpected::Signed(value), &self))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ServiceErrorCode::from_str(value)
            .map_err(|_e| de::Error::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for ServiceErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(ErrorCodeVisitor)
    }
}

/// serialize service error code into integer
impl Serialize for ServiceErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = self
            .to_u64()
            .expect("The enum type holds an invalid value => Primitive crate is bugged");
        serializer.serialize_u64(val)
    }
}

/// parses a int which is a string to a proper service error code type
impl FromStr for ServiceErrorCode {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map_err(|_| MessageBirdError::ParseError)
            .and_then(|x| Self::from_u64(x).ok_or(MessageBirdError::ParseError))
    }
}

impl ServiceErrorCode {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            ServiceErrorCode::RequestNotAllowed => "Request not allowed",
            ServiceErrorCode::MissingParameters => "Missing params",
            ServiceErrorCode::InvalidParameters => "Invalid params",
            ServiceErrorCode::NotFound => "Not found",
            ServiceErrorCode::BadRequest => "Bad request",
            ServiceErrorCode::NotEnoughBalance => "Not enough balance",
            ServiceErrorCode::EndpointNotFound => "API not found",
            ServiceErrorCode::InternalError => "Internal error",
            _ => "unknown error",
        }
    }
}

/// Error as returned from the MessageBird API
///
/// Contains an error code and some additional meta parameters
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServiceError {
    code: ServiceErrorCode,
    // this is not the same as the stringification of the code,
    // may contain additional details as to 'why', see the test mod futher down
    description: String,
    parameter: Option<String>,
}

impl ServiceError {
    pub fn new(code: ServiceErrorCode, description: String, parameter: Option<String>) -> Self {
        Self {
            code,
            description,
            parameter,
        }
    }
}

impl FromStr for ServiceError {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s).map_err(|_e| MessageBirdError::ParseError)
    }
}

impl ToString for ServiceError {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ops::Deref;

    static RAW_ERRORS: &str = r#"
{
  "errors":[
    {
      "code": 2,
      "description": "Request not allowed (incorrect access_key)",
      "parameter": "access_key"
    }
  ]
}
"#;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Wrapper {
        errors: Vec<ServiceError>,
    }

    impl Default for Wrapper {
        fn default() -> Self {
            Self { errors: vec![] }
        }
    }

    impl Wrapper {
        fn new(errors: Vec<ServiceError>) -> Self {
            Self { errors }
        }
    }

    #[allow(dead_code)]
    impl Deref for Wrapper {
        type Target = Vec<ServiceError>;
        fn deref(&self) -> &Self::Target {
            &self.errors
        }
    }

    lazy_static! {
        static ref ERRVEC: Vec<ServiceError> = {
            let v = vec![ServiceError::new(
                ServiceErrorCode::RequestNotAllowed,
                "Request not allowed (incorrect access_key)".to_string(),
                Some("access_key".to_string()),
            )];
            println!("raw vec of service errors {:?}", v);
            v
        };
    }

    serde_roundtrip!(serde_service_errors, Wrapper, Wrapper::new(ERRVEC.to_vec()));
    deser_roundtrip!(deser_service_errors, Wrapper, RAW_ERRORS);
}
