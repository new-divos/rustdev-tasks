use std::fmt::Debug;

pub mod error;
pub mod wrapper;

///
/// Типаж, описывающий преобразование одной последовательности
/// байт в другую.
///
pub trait Transformer: Debug {
    ///
    /// Преобразовать одну последовательности байт
    /// в другую последовательность байт.
    ///
    fn transform(&mut self, bytes: &[u8]) -> Result<&[u8], error::Error>;
}
