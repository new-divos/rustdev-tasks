use std::io;

use bincode::{self, Options};
use serde::{de, Deserialize, Serialize};
use tokio::net::TcpStream;

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
}

///
/// Версия протокола.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolVersion {
    #[serde(rename = "1.0")]
    V1_0,
}

// Асинхронно прочитать заданное количество байт.
pub(crate) async fn read_exact_async(s: &TcpStream, buf: &mut [u8]) -> io::Result<()> {
    let mut red = 0;
    while red < buf.len() {
        s.readable().await?;
        match s.try_read(&mut buf[red..]) {
            Ok(0) => break,
            Ok(n) => {
                red += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

// Асинхронно записать заданное количество байт.
pub(crate) async fn write_all_async(stream: &TcpStream, buf: &[u8]) -> io::Result<()> {
    let mut written = 0;

    while written < buf.len() {
        stream.writable().await?;

        match stream.try_write(&buf[written..]) {
            Ok(0) => break,
            Ok(n) => {
                written += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

// Отправить сообщение.
pub(crate) async fn send_message<M: Message + Serialize>(
    message: M,
    stream: &TcpStream,
) -> Result<(), SendError> {
    let bytes = M::TYPE.to_be_bytes();
    write_all_async(stream, &bytes).await?;

    let data = bincode::options().with_big_endian().serialize(&message)?;
    let size = data.len() as u32;
    let bytes = size.to_be_bytes();
    write_all_async(stream, &bytes).await?;
    write_all_async(stream, data.as_ref()).await?;

    Ok(())
}

// Получить сообщение.
pub(crate) async fn recv_message<M: Message + de::DeserializeOwned>(
    stream: &TcpStream,
) -> Result<Box<M>, RecvError> {
    let mut bytes = [0u8; 2];
    read_exact_async(stream, &mut bytes).await?;
    let message_type = u16::from_be_bytes(bytes);
    if message_type != M::TYPE {
        return Err(RecvError::BadType(message_type));
    }

    let mut bytes = [0u8; 4];
    read_exact_async(stream, &mut bytes).await?;
    let len = u32::from_be_bytes(bytes);

    let mut data = vec![0u8; len as _];
    read_exact_async(stream, &mut data).await?;
    let message = bincode::options()
        .with_big_endian()
        .deserialize(&data[..])?;

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
