use std::fmt;

use uuid::Uuid;

use crate::device::Device;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_thermometer_test() {
        let thermometer1 = SmartThermometer::new("Thermometer1", 20.0);
        assert_eq!(thermometer1.name.as_str(), "Thermometer1");
        assert_eq!(thermometer1.temperature, 20.0);
    }
}
