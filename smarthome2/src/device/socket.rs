use std::fmt;

use uuid::Uuid;

use crate::device::Device;

///
/// Структура, описывающая взаимодействие с "умной" розеткой.
///
pub struct SmartSocket {
    ///
    /// Идентификатор "умной" розетки.
    ///
    id: Uuid,

    ///
    /// Имя "умной" розетки.
    ///
    name: String,

    ///
    /// Текущее состояние розетки.
    ///
    state: bool,

    ///
    /// Потребляемая мощность.
    ///
    power: f64,
}

impl fmt::Display for SmartSocket {
    ///
    /// Получить информацию об "умной" розетке с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = vec![format!(
            "умная розетка {} ({}). Состояние: ",
            self.name, self.id
        )];

        if self.state {
            v.push(format!(
                "включена, потребляемая мощность {} Вт.",
                self.power
            ));
        } else {
            v.push("выключена.".to_string());
        }

        write!(f, "{}", v.join(""))
    }
}

impl Device for SmartSocket {
    ///
    /// Получить идентификатор "умной" розетки.
    ///
    fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умной" розетки.
    ///
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl SmartSocket {
    ///
    /// Создать "умную" розетку в выключенном состоянии.
    ///
    pub fn new(name: &str) -> Self {
        SmartSocket {
            id: Uuid::new_v4(),
            name: name.to_string(),
            state: false,
            power: 0.0,
        }
    }

    ///
    /// Включить "умную" розетку.
    ///
    pub fn switch_on(&mut self) {
        self.state = true;
    }

    ///
    /// Выключить "умную" розетку.
    ///
    pub fn switch_off(&mut self) {
        self.state = false;
    }

    ///
    /// Проверить, включена ли "умная" розетка.
    ///
    pub fn is_switched_on(&self) -> bool {
        self.state
    }

    ///
    /// Получить текущее значение потребляемой мощности.
    ///
    pub fn power(&self) -> Option<f64> {
        if self.state {
            Some(self.power)
        } else {
            None
        }
    }

    ///
    /// Подключить нагрузку с заданной мощностью.
    ///
    pub fn plug(&mut self, power: f64) {
        self.power = power;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_socket_test() {
        let mut socket1 = SmartSocket::new("Socket1");
        assert_eq!(socket1.name.as_str(), "Socket1");
        assert!(!socket1.state);
        assert_eq!(socket1.power, 0.0);

        socket1.switch_on();
        assert!(socket1.state);

        socket1.plug(1000.0);
        assert_eq!(socket1.power, 1000.0);

        socket1.switch_off();
        assert!(!socket1.state);
    }
}
