use futures::task::SpawnError;

#[derive(Debug)]
pub(crate) enum Error {
    Sending,
    SpawnError,
}

impl From<SpawnError> for Error {
    fn from(_: SpawnError) -> Self {
        Error::SpawnError
    }
}

impl From<u64> for Error {
    fn from(_: u64) -> Self {
        Error::Sending
    }
}
