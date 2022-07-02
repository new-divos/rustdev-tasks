use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
};

use serde::{de, Serialize};

use crate::{
    error::{BindError, ConnectionError, RecvError, SendError},
    protocol::{consts::MASK, mask, recv_message, send_message, Message},
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
    pub fn bind<A>(addrs: A) -> Result<Self, BindError>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addrs)?;
        Ok(Self { listener })
    }

    ///
    /// Блокирующий итератор для входящих соединений.
    ///
    pub fn incoming(&self) -> impl Iterator<Item = Result<Connection, ConnectionError>> + '_ {
        self.listener.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(ConnectionError::Io(e)),
        })
    }

    // Подтвердить handshake.
    fn try_handshake(mut stream: TcpStream) -> Result<Connection, ConnectionError> {
        let mut bytes = [0u8; 32];
        stream.read_exact(&mut bytes)?;
        let bytes = mask(bytes, MASK);
        stream.write_all(&bytes)?;

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
    pub fn send<M: Message + Serialize>(&mut self, response: M) -> Result<(), SendError> {
        send_message(response, &mut self.stream)
    }

    ///
    /// Получить запрос от клиента.
    ///
    #[inline]
    pub fn recv<M: Message + de::DeserializeOwned>(&mut self) -> Result<Box<M>, RecvError> {
        recv_message(&mut self.stream)
    }

    ///
    /// Получить адрес подключенного клиента.
    ///
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
