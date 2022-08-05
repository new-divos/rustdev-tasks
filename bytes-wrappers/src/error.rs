use thiserror::Error;

///
/// Ошибка преобразования последовательности байт.
///
#[derive(Error, Debug)]
pub enum Error {
    #[error("illegal transformer state")]
    IllegalStateError,

    #[error("CRC32 mismatch {0:#x} and {1:#x} error")]
    CRC32MismatchError(u32, u32),
}
