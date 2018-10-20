use super::*;

use std::str::FromStr;

/// error codes
///
/// Error codes as returned as part of a response from the service in the payload.
/// These are NOT http status codes.
#[derive(Primitive, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

impl ServiceErrorCode {
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServiceError {
    #[serde(flatten)]
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


    #[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
    struct Wrapper {
        errors : Vec<ServiceError>
    }

    impl Default for Wrapper {
        fn default() -> Self {
            Self {
                errors : vec![]
            }
        }
    }

    impl Wrapper {
        fn new(errors : Vec<ServiceError>) -> Self {
            Self {
                errors
            }
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
