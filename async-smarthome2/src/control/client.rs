use tokio::net::ToSocketAddrs;

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
    pub async fn connect<A>(addrs: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            client: Client::connect(addrs).await?,
        })
    }

    ///
    /// Отправить запрос серверу и получить ответ от него.
    ///
    pub async fn request(&self, req: ControlRequest) -> Result<Box<ControlResponse>, RequestError> {
        let response: Box<ControlResponse> = self.client.request(req).await?;

        if let ControlResponseData::Error(message) = response.data {
            Err(RequestError::ServerError(message))
        } else {
            Ok(response)
        }
    }
}
