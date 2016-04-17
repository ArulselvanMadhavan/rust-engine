use std::error;
use std::fmt;
use std::sync::TryLockError;

enum ServerError{
    LockBusy(TryLockError)
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::LockBusy(ref err) => write!(f, "LockBusy error: {}", err),
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            ServerError::LockBusy(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::LockBusy(ref err) => Some(err),
        }
    }
}
