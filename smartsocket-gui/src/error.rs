use thiserror::Error;

///
/// Ошибка при работе с программой.
///
#[derive(Error, Debug)]
pub enum Error {
    #[error("application initialization error")]
    AppInitError,

    #[error("IO error {0}")]
    IOError(#[from] std::io::Error),

    #[error("configuration deserialization error {0}")]
    ConfigParseError(#[from] toml::de::Error),

    #[error("configuration serialization error {0}")]
    ConfigSerializeError(#[from] toml::ser::Error),
}
