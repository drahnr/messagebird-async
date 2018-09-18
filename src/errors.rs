// pub type MessageBirdResult<T> = std::result::Result<T, MessageBirdError>;

#[derive(Debug, Fail)]
pub enum MessageBirdError {
    #[fail(display = "invalid json format: {}", chunk)]
    FormatError { chunk: String },

    #[fail(display = "invalid paramter for type: {}", msg)]
    TypeError { msg: String },

    #[fail(display = "service return code: {}", code)]
    ServiceError { code: u16 },

    #[fail(display = "parsing failed")]
    ParseError,

    #[fail(display = "sending request failed")]
    RequestError,
}
