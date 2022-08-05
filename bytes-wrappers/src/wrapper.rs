use crc::{Crc, CRC_32_ISO_HDLC};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::{error::Error, Transformer};

///
/// Базовый преобразователь последовательности байт.
///
#[derive(Debug)]
pub struct BaseTransformer {
    // Данные после преобразования.
    data: Option<Vec<u8>>,
}

impl Default for BaseTransformer {
    ///
    /// Создать объект базового преобразователя по умолчанию.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer for BaseTransformer {
    ///
    /// Базовый преобразователь не выполняет никаких преобразований,
    /// а просто копирует последовательность байт, требующую преобразования.
    ///
    fn transform(&mut self, bytes: &[u8]) -> Result<&[u8], Error> {
        self.data = Some(Vec::from(bytes));

        self.data.as_deref().ok_or(Error::IllegalStateError)
    }
}

impl BaseTransformer {
    ///
    /// Создать экземпляр базового преобразователя последовательности байт.
    ///
    #[inline]
    pub fn new() -> Self {
        Self { data: None }
    }
}

///
/// Преобразователь, считающий CRC32 для последовательности байт и добавляющий
/// полученную сумму в конец последовательности байт.
///
#[derive(Debug)]
pub struct CRC32Wrapper<T: Transformer> {
    // Данные после преобразования.
    data: Option<Vec<u8>>,
    // Экземпляр внутреннего преобразователя.
    inner: T,
}

impl<T: Transformer> Transformer for CRC32Wrapper<T> {
    ///
    /// Преобразовать одну последовательности байт
    /// в другую последовательность байт.
    ///
    fn transform(&mut self, bytes: &[u8]) -> Result<&[u8], Error> {
        let mut data = Vec::from(self.inner.transform(bytes)?);

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(data.as_ref());
        let checksum = digest.finalize();

        data.extend(checksum.to_be_bytes());
        self.data = Some(data);

        self.data.as_deref().ok_or(Error::IllegalStateError)
    }
}

impl<T: Transformer> CRC32Wrapper<T> {
    ///
    /// Создать объект преобразователя с заданным внутренним
    /// преобразователем.
    ///
    #[inline]
    pub fn new(transformer: T) -> Self {
        Self {
            data: None,
            inner: transformer,
        }
    }
}

///
/// Преобразователь, удаляющий последдние четыре байта из
/// последовательности байт и выполняющий валидацию полученных данных
/// с использованием контрольной суммы CRC32.
///
#[derive(Debug)]
pub struct CRC32Unwrapper<T: Transformer> {
    // Данные после преобразования.
    data: Option<Vec<u8>>,
    // Экземпляр внутреннего преобразователя.
    inner: T,
}

impl<T: Transformer> Transformer for CRC32Unwrapper<T> {
    ///
    /// Преобразовать одну последовательности байт
    /// в другую последовательность байт.
    ///
    fn transform(&mut self, bytes: &[u8]) -> Result<&[u8], Error> {
        let mut data = Vec::from(self.inner.transform(bytes)?);
        let len = data.len();

        let checksum1 = u32::from_be_bytes(data[len - 4..].try_into().unwrap());

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(&data[..len - 4]);
        let checksum2 = digest.finalize();

        if checksum1 != checksum2 {
            return Err(Error::CRC32MismatchError(checksum1, checksum2));
        }

        data.resize(len - 4, 0u8);
        self.data = Some(data);

        self.data.as_deref().ok_or(Error::IllegalStateError)
    }
}

impl<T: Transformer> CRC32Unwrapper<T> {
    ///
    /// Создать объект преобразователя с заданным внутренним
    /// преобразователем.
    ///
    #[inline]
    pub fn new(transformer: T) -> Self {
        Self {
            data: None,
            inner: transformer,
        }
    }
}

///
/// Преобразователь, выполняющий перестановку четных и нечетных
/// байт в последовательности байт.
///
#[derive(Debug)]
pub struct SwapTransformer<T: Transformer> {
    // Данные после преобразования.
    data: Option<Vec<u8>>,
    // Экземпляр внутреннего преобразователя.
    inner: T,
}

impl<T: Transformer> Transformer for SwapTransformer<T> {
    ///
    /// Преобразовать одну последовательности байт
    /// в другую последовательность байт.
    ///
    fn transform(&mut self, bytes: &[u8]) -> Result<&[u8], Error> {
        let mut data = Vec::from(self.inner.transform(bytes)?);
        let len = data.len();

        for i in 0..len >> 1 {
            data.swap(i << 1, (i << 1) + 1);
        }
        self.data = Some(data);

        self.data.as_deref().ok_or(Error::IllegalStateError)
    }
}

impl<T: Transformer> SwapTransformer<T> {
    ///
    /// Создать объект преобразователя с заданным внутренним
    /// преобразователем.
    ///
    #[inline]
    pub fn new(transformer: T) -> Self {
        Self {
            data: None,
            inner: transformer,
        }
    }
}

///
/// Преобразователь, осуществляющий наложение псевдослучайной
/// последовательности на исходную последовательность байт
/// с использованием операции XOR.
///
#[derive(Debug)]
pub struct GammaTransformer<T: Transformer> {
    // Данные после преобразования.
    data: Option<Vec<u8>>,
    // Экземпляр внутреннего преобразователя.
    inner: T,
    // Порождающее значение генератора случайных чисел.
    seed: u64,
}

impl<T: Transformer> Transformer for GammaTransformer<T> {
    ///
    /// Преобразовать одну последовательности байт
    /// в другую последовательность байт.
    ///
    fn transform(&mut self, bytes: &[u8]) -> Result<&[u8], Error> {
        let mut data = Vec::from(self.inner.transform(bytes)?);

        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        for value in data.iter_mut() {
            *value ^= rng.gen::<u8>();
        }
        self.data = Some(data);

        self.data.as_deref().ok_or(Error::IllegalStateError)
    }
}

impl<T: Transformer> GammaTransformer<T> {
    ///
    /// Создать объект преобразователя с заданным внутренним
    /// преобразователем и порождающим значением генератора случайных
    /// чисел.
    ///
    #[inline]
    pub fn new(transformer: T, seed: u64) -> Self {
        Self {
            data: None,
            inner: transformer,
            seed,
        }
    }
}
