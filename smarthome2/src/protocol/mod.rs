use std::io::{Read, Write};

use bincode;
use serde::{de, Serialize};

use crate::error::{RecvError, SendError};

pub mod client;
pub mod consts;
pub mod server;

///
/// Типаж для отправки и получения сообщений по сети.
///
pub trait Message {
    ///
    /// Идентификатор типа сообщения.
    ///
    const TYPE: u16;

    ///
    /// Флаги сообщения.
    ///
    const FLAGS: u8;
}

// Отправить сообщение.
pub(crate) fn send_message<M: Message + Serialize, W: Write>(
    message: M,
    mut writer: W,
) -> Result<(), SendError> {
    let bytes = M::TYPE.to_be_bytes();
    writer.write_all(&bytes)?;
    let bytes = M::FLAGS.to_be_bytes();
    writer.write_all(&bytes)?;

    let data = bincode::serialize(&message)?;
    let size = data.len() as u32;
    let bytes = size.to_be_bytes();
    writer.write_all(&bytes)?;
    writer.write_all(data.as_ref())?;

    Ok(())
}

// Получить сообщение.
pub(crate) fn recv_message<M: Message + de::DeserializeOwned, R: Read>(
    mut reader: R,
) -> Result<Box<M>, RecvError> {
    let mut bytes = [0u8; 2];
    reader.read_exact(&mut bytes)?;
    let message_type = u16::from_be_bytes(bytes);
    if message_type != M::TYPE {
        return Err(RecvError::BadType(message_type));
    }

    let mut bytes = [0u8; 1];
    reader.read_exact(&mut bytes)?;
    if (bytes[0] & M::FLAGS) == 0 {
        return Err(RecvError::BadFlags(bytes[0]));
    }

    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    let len = u32::from_be_bytes(bytes);

    let mut data = vec![0u8; len as _];
    reader.read_exact(&mut data)?;
    let message = bincode::deserialize(&data[..])?;

    Ok(Box::new(message))
}

// Маскировать бинарные данные.
pub(crate) fn mask<const N: usize>(data: [u8; N], mask: &[u8]) -> [u8; N] {
    let mut result = [0u8; N];
    for (idx, (&v1, &v2)) in data.iter().zip(mask.iter().cycle()).enumerate() {
        result[idx] = v1 ^ v2;
    }

    result
}
