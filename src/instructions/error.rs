use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
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

    #[error("Downcast Error")]
    DowncastError,

    #[error("Label '{0}' already exists, all labels should be unique")]
    LabelExist(String),

    #[error("Symbol '{0}' already exists, all symbols should be unique")]
    SymbolExist(String),

    // reserved
    #[error("symbol '{0}' is reserved for internal use, please use another symbol name")]
    SymbolReserved(String),
}
