use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Downcast Error")]
    DowncastError,
}
