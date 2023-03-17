use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown register: R{0}")]
    UnknownRegister(String),

    #[error("Unknown flag: {0}")]
    UnknownFlag(String),

    #[error("Unknown io mode: {0}")]
    UnknownIoMode(String),

    #[error("Unknown marker: {0}")]
    UnknownMarker(String),

    #[error("Unknown symbol: {0}")]
    UnknownSymbol(String),

    #[error("Unknown label: {0}")]
    UnknownLabel(String),
}
