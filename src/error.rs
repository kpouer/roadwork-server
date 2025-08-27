use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("SqlxError {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("InvalidSql")]
    NetworkError(#[from] std::io::Error),
}