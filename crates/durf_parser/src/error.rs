//! durf parsing error.

/// durf error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Depth exceeded during parse.")]
    DepthExceeded,
    #[error("Feature is incomplete.")]
    Todo,
}
