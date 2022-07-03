use std::fmt;

use serde::{Deserialize, Serialize};

use crate::control::protocol::Message;

///
/// Сообщение для обмена тестовыми данными.
///
#[derive(Serialize, Deserialize)]
pub struct TextMessage {
    text: String,
}

impl fmt::Display for TextMessage {
    ///
    /// Выполнить форматирование сообщения.
    /// 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Message for TextMessage {
    ///
    /// Идентификатор типа сообщения.
    /// 
    const TYPE: u16 = 0xFFFF;
}

impl TextMessage {
    ///
    /// Создать сообщение с заданным текстом.
    /// 
    pub fn new<D: AsRef<str>>(text: D) -> Self {
        Self {
            text: text.as_ref().to_owned(),
        }
    }
}
