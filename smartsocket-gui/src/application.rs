use iced::{
    executor,
    widget::{button, Button, Column, Container, Text},
    Alignment, Application, Command, Element, Length,
};

use crate::config::Config;
use smarthome2::device::{
    socket::{RemoteSmartSocket, SwitchOffEvent, SwitchOnEvent},
    Device, DeviceState, StateEvent,
};

///
/// Доступные сообщения приложения.
///
#[derive(Debug, Clone)]
pub enum Message {
    ///
    /// Подключиться к серверу умной розетки.
    ///
    Connect,
    ///
    /// Отключиться от сервера умной розетки.
    ///
    Disconnect,
    ///
    /// Включить умную розетку.
    ///
    TurnOn,
    ///
    /// Выключить умную розетку.
    ///
    TurnOff,
}

///
/// Структура, описывающая приложение.
///
pub struct SmartSocketClient {
    // Конфигурация программы.
    config: Config,

    // Объект для управления удаленной розеткой.
    socket: Option<RemoteSmartSocket>,

    // Состояние удаленной розетки.
    socket_state: Option<DeviceState>,

    // Состояние кнопки "Подключить".
    connect_btn_state: button::State,

    // Состояние кнопки "Отключить".
    disconnect_btn_state: button::State,

    // Состояние кнопки "Вкючить"
    switch_on_btn_state: button::State,

    // Состояние кнопки "Выключить".
    switch_off_btn_state: button::State,

    // Сообщение об ошибке.
    error_msg: Option<String>,
}

impl Application for SmartSocketClient {
    type Executor = executor::Default;

    ///
    /// Доступные сообщения для запуска приложения.
    ///
    type Message = Message;

    ///
    /// Данные необходимые для инициализации приложения.
    ///
    type Flags = Config;

    ///
    /// Инициализировать экземпляр приложения.
    ///
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                config: flags,
                socket: None,
                socket_state: None,

                connect_btn_state: button::State::new(),
                disconnect_btn_state: button::State::new(),
                switch_on_btn_state: button::State::new(),
                switch_off_btn_state: button::State::new(),
                error_msg: None,
            },
            Command::none(),
        )
    }

    ///
    /// Получить заголовок приложения.
    ///
    #[inline]
    fn title(&self) -> String {
        if self.socket.is_some() {
            format!(
                "Управление умной розеткой. Подключенно к {}",
                self.config.server_addrs()
            )
        } else {
            "Управление умной розеткой. Отключено".to_string()
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Connect => match RemoteSmartSocket::connect(self.config.server_addrs()) {
                Ok(mut socket) => match socket.notify(&StateEvent::new()) {
                    Ok(state) => {
                        self.socket_state = Some(state);
                        self.socket = Some(socket);
                        self.error_msg = None;
                    }
                    Err(error) => {
                        self.error_msg = Some(format!("Ошибка запроса состояния: {}", error))
                    }
                },
                Err(error) => self.error_msg = Some(format!("Ошибка подключения: {}", error)),
            },

            Message::Disconnect => {
                self.socket_state = None;
                self.socket = None;
                self.error_msg = None;
            }

            Message::TurnOn => {
                if let Some(ref mut socket) = self.socket {
                    match socket.notify(&SwitchOnEvent::new()) {
                        Ok(state) => {
                            self.socket_state = Some(state);
                            self.error_msg = None;
                        }
                        Err(error) => {
                            self.error_msg = Some(format!("Ошибка включения розетки: {}", error))
                        }
                    }
                }
            }

            Message::TurnOff => {
                if let Some(ref mut socket) = self.socket {
                    match socket.notify(&SwitchOffEvent::new()) {
                        Ok(state) => {
                            self.socket_state = Some(state);
                            self.error_msg = None;
                        }
                        Err(error) => {
                            self.error_msg = Some(format!("Ошибка выключения розетки: {}", error))
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut column;
        if self.socket.is_some() {
            column = Column::new();
            if let Some(ref state) = self.socket_state {
                if let Some(enabled) = state.enabled() {
                    if enabled {
                        let socket_text = Text::new("Розетка включена").size(20);
                        let power_text = if let Some(power) = state.power() {
                            Text::new(format!("Потребляемая мощность {} Вт.", power))
                                .color([0.0, 0.0, 1.0])
                                .size(16)
                        } else {
                            Text::new("Потребляемая мощность неизвестна")
                                .color([1.0, 0.0, 0.0])
                                .size(16)
                        };
                        let switch_off_btn =
                            Button::new(&mut self.switch_off_btn_state, Text::new("Выключить"))
                                .padding(10)
                                .on_press(Message::TurnOff);

                        column = column
                            .push(socket_text)
                            .push(power_text)
                            .push(switch_off_btn);
                    } else {
                        let socket_text = Text::new("Розетка выключена").size(20);
                        let switch_on_btn =
                            Button::new(&mut self.switch_on_btn_state, Text::new("Включить"))
                                .padding(10)
                                .on_press(Message::TurnOn);

                        column = column.push(socket_text).push(switch_on_btn);
                    }
                }
            }

            let disconnect_btn =
                Button::new(&mut self.disconnect_btn_state, Text::new("Отключить"))
                    .padding(10)
                    .on_press(Message::Disconnect);

            column = column.push(disconnect_btn);
        } else {
            let connect_btn = Button::new(&mut self.connect_btn_state, Text::new("Подключить"))
                .padding(10)
                .on_press(Message::Connect);

            column = Column::new().push(connect_btn);
        }

        if let Some(ref error_msg) = self.error_msg {
            let error_text = Text::new(error_msg.as_str())
                .color([1.0, 0.0, 0.0])
                .size(12);
            column = column.push(error_text);
        }

        column = column
            .padding(10)
            .spacing(20)
            .align_items(Alignment::Center);
        Container::new(column)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
