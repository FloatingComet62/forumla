pub mod cache;
pub mod client;

#[derive(Debug)]
pub enum Errors {
    RequestFailed,
    InvalidResponseBody,

    FailedToSetupCache,
    FailedToSaveCache,
    FailedToLoadCache,
}

pub type Result<T> = std::result::Result<T, Errors>;
