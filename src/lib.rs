pub mod cache;
pub mod client;
pub mod params;
pub mod schemas;

#[derive(Debug)]
pub enum Errors {
    RequestFailed,
    InvalidResponseBody,
    InvalidResponseSchema,

    FailedToSetupCache,
    FailedToSaveCache,
    FailedToLoadCache,
}

pub type Result<T> = std::result::Result<T, Errors>;
