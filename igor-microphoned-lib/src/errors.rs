
use thiserror::Error as ThisError;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    DeviceNotFoundError,
    DeviceError,
    HostNotFoundError
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("the requested device is unavailable")]
    DeviceNotFoundError,
    #[error("device error")]
    DeviceError(#[from] cpal::DevicesError),
    #[error("host not found")]
    HostNotFoundError(#[from] cpal::HostUnavailable)
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            _ => self.kind() == other.kind(),
        }
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::DeviceNotFoundError => ErrorKind::DeviceNotFoundError,
            Error::DeviceError(_) => ErrorKind::DeviceError,
            Error::HostNotFoundError(_) => ErrorKind::HostNotFoundError,
        }
    }
}
