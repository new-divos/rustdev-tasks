use std::net::ToSocketAddrs;

use crate::{
    control::{
        message::{ControlRequest, ControlResponse, ControlResponseData},
        protocol::client::Client,
    },
    error::{ConnectionError, RequestError},
};

///
/// Клиент подсистемы управления "умного" дома.
///
pub struct ControlClient {
    client: Client,
}

impl ControlClient {
    ///
    /// Подключиться к серверу с заданным адресом.
    ///
    pub fn connect<A>(addrs: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            client: Client::connect(addrs)?,
        })
    }

    ///
    /// Отправить запрос серверу и получить ответ от него.
    ///
    pub fn request(&mut self, req: ControlRequest) -> Result<Box<ControlResponse>, RequestError> {
        let response: Box<ControlResponse> = self.client.request(req)?;

        if let ControlResponseData::Error(message) = response.data {
            Err(RequestError::Srv(message))
        } else {
            Ok(response)
        }
    }
}
