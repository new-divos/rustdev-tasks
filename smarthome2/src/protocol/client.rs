use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use rand::{self, Rng};
use serde::{de, Serialize};

use crate::{
    error::{ConnectionError, RequestError},
    protocol::{consts::MASK, mask, recv_message, send_message, Message},
};

///
/// Представляет клиент для обмена сообщениями.
///
pub struct Client {
    stream: TcpStream,
}

impl Client {
    ///
    /// Подключиться к серверу с заданным адресом.
    ///
    pub fn connect<A>(addrs: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handshake(stream)
    }

    ///
    /// Отправить запрос серверу и получить ответ от него.
    ///
    pub fn request<R, S>(&mut self, req: R) -> Result<Box<S>, RequestError>
    where
        R: Message + Serialize,
        S: Message + de::DeserializeOwned,
    {
        send_message(req, &mut self.stream)?;
        let response = recv_message(&mut self.stream)?;

        Ok(response)
    }

    // Подтвердить handshake.
    fn try_handshake(mut stream: TcpStream) -> Result<Self, ConnectionError> {
        let data = rand::thread_rng().gen::<[u8; 32]>();
        stream.write_all(&data)?;

        let mut bytes = [0u8; 32];
        stream.read_exact(&mut bytes)?;

        let bytes = mask(bytes, MASK);
        if bytes != data {
            return Err(ConnectionError::BadHandshake);
        }

        Ok(Self { stream })
    }
}
