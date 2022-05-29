use std::fmt;

use uuid::Uuid;

///
/// Типаж, описывающий устройство.
///
pub trait Device: fmt::Display {
    ///
    /// Получить идентификатор устройства.
    ///
    fn id(&self) -> Uuid;

    ///
    /// Получить имя устройства.
    ///
    fn name(&self) -> &str;
}

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

///
/// Структура, описывающая взаимодействие с "умным" термометром.
///
pub struct SmartThermometer {
    ///
    /// Идентификатор "умного" термометра.
    ///
    id: Uuid,

    ///
    /// Имя "умного" термометра.
    ///
    name: String,

    ///
    /// Текущее значение температуры.
    ///
    temperature: f64,
}

impl fmt::Display for SmartThermometer {
    ///
    /// Получить информацию об "умном" термометре с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "умный термометр {} ({}). Температура: {} °C.",
            self.name, self.id, self.temperature
        )
    }
}

impl Device for SmartThermometer {
    ///
    /// Получить идентификатор "умного" термометра.
    ///
    fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умного" термометра.
    ///
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl SmartThermometer {
    ///
    /// Создать термометр с заданным значением температуры.
    ///
    pub fn new(name: &str, temperature: f64) -> Self {
        SmartThermometer {
            id: Uuid::new_v4(),
            name: name.to_string(),
            temperature,
        }
    }

    ///
    /// Получить текущее значение температуры.
    ///
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}
