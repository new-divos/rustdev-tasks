use rand::{self, Rng};
use serde::{de, Serialize};
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::{
    control::protocol::{consts::MASK, mask, recv_message, send_message, Message},
    error::{ConnectionError, RequestError},
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
    pub async fn connect<A>(addrs: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Self::try_handshake(stream).await
    }

    ///
    /// Отправить запрос серверу и получить ответ от него.
    ///
    pub async fn request<R, S>(&self, req: R) -> Result<Box<S>, RequestError>
    where
        R: Message + Serialize,
        S: Message + de::DeserializeOwned,
    {
        send_message(req, &self.stream).await?;
        let response = recv_message(&self.stream).await?;

        Ok(response)
    }

    // Подтвердить handshake.
    async fn try_handshake(stream: TcpStream) -> Result<Self, ConnectionError> {
        let data = rand::thread_rng().gen::<[u8; 32]>();
        super::write_all_async(&stream, &data).await?;

        let mut bytes = [0u8; 32];
        super::read_exact_async(&stream, &mut bytes).await?;

        let bytes = mask(bytes, MASK);
        if bytes != data {
            return Err(ConnectionError::BadHandshake);
        }

        Ok(Self { stream })
    }
}
