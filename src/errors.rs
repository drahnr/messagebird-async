// pub type MessageBirdResult<T> = std::result::Result<T, MessageBirdError>;

#[derive(Debug, Fail)]
pub enum MessageBirdError {
    #[fail(display = "invalid json format: {}", chunk)]
    FormatError { chunk: String },

    #[fail(display = "invalid paramter for type: {}", msg)]
    TypeError { msg: String },

    #[fail(display = "service return code: {}/{}/{:?}", code, description, parameter)]
    ServiceError { code: u16, description : String, parameter : Option<String> },

    #[fail(display = "parsing failed")]
    ParseError,

    #[fail(display = "sending request failed")]
    RequestError,

    #[fail(display = "did not find a valid access key {}", msg)]
    AccessKeyError { msg: String },
}
