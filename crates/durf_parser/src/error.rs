/// durf error type.
#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// Depth exceeded during parse.
    DepthExceeded,
    /// Feature is incomplete.
    Todo,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DurfError {:?}", self)
    }
}
