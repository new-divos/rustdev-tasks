use std::{io, net::SocketAddr};

use serde::{de, Serialize};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::{
    control::protocol::{consts::MASK, mask, recv_message, send_message, Message},
    error::{BindError, ConnectionError, RecvError, SendError},
};

///
/// Представляет сервер для обмена сообщениями.
///
pub struct Server {
    listener: TcpListener,
}

impl Server {
    ///
    /// Выполнить привязку сервера к сокету.
    ///
    pub async fn bind<A>(addrs: A) -> Result<Self, BindError>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addrs).await?;
        Ok(Self { listener })
    }

    ///
    /// Получить входящее соединение.
    ///
    pub async fn accept(&self) -> Result<Connection, ConnectionError> {
        let (connection, _) = self.listener.accept().await?;
        Self::try_handshake(connection).await
    }

    // Подтвердить handshake.
    async fn try_handshake(stream: TcpStream) -> Result<Connection, ConnectionError> {
        let mut bytes = [0u8; 32];
        super::read_exact_async(&stream, &mut bytes).await?;
        let bytes = mask(bytes, MASK);
        super::write_all_async(&stream, &bytes).await?;

        Ok(Connection { stream })
    }
}

///
/// Представляет соединение с клиентом.
///
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    ///
    /// Отправить ответ сервера.
    ///
    #[inline]
    pub async fn send<M: Message + Serialize>(&self, response: M) -> Result<(), SendError> {
        send_message(response, &self.stream).await
    }

    ///
    /// Получить запрос от клиента.
    ///
    #[inline]
    pub async fn recv<M: Message + de::DeserializeOwned>(&self) -> Result<Box<M>, RecvError> {
        recv_message(&self.stream).await
    }

    ///
    /// Получить адрес подключенного клиента.
    ///
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
