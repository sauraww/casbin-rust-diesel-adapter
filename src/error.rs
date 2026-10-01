use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};

use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    PoolError(PoolError),
    DieselError(DieselError),
}

impl Error {
    /// Whether the database rejected a write because it is read-only (e.g. a replica or standby).
    pub fn is_read_only(&self) -> bool {
        matches!(
            self,
            Error::DieselError(DieselError::DatabaseError(
                DatabaseErrorKind::ReadOnlyTransaction,
                _
            ))
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            PoolError(pool_err) => pool_err.fmt(f),
            DieselError(diesel_error) => diesel_error.fmt(f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;

        match self {
            PoolError(pool_err) => Some(pool_err),
            DieselError(diesel_error) => Some(diesel_error),
        }
    }
}

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Self {
        Error::PoolError(err)
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        Error::DieselError(err)
    }
}
