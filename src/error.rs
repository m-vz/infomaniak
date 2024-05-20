#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    /// The requested URI needs the client to have an API token with the right scopes
    Unauthorised,
    /// Access is forbidden to the requested resource
    Forbidden,
    /// The server can not find the requested resource
    NotFound,
    /// The request wass well formatted, but the request content contains semantic errors
    UnprocessableEntity,
    /// The request was not completed. The server met an unexpected condition
    InternalServerError,
    /// The server sent an undocumented response code
    UnexpectedResponseCode,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}
