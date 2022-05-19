///
/// Структура, описывающая взаимодействие с "умной" розеткой.
///
pub struct SmartSocket {
    ///
    /// Текущее состояние розетки.
    ///
    state: bool,

    ///
    /// Потребляемая мощность.
    ///
    power: f64,
}

impl SmartSocket {
    ///
    /// Создать "умную" розетку в выключенном состоянии.
    ///
    pub fn new() -> Self {
        SmartSocket {
            state: false,
            power: 0.0,
        }
    }

    ///
    /// Создать "умную" розетку во включенном состоянии.
    ///
    pub fn with_power(power: f64) -> Self {
        SmartSocket { state: true, power }
    }

    ///
    /// Отобразить информацию об "умной" розетке.
    ///
    pub fn info(&self) -> String {
        if self.state {
            format!(
                "Розетка включена. Потребляемая мощность: {} Вт.",
                self.power
            )
        } else {
            String::from("Розетка выключена")
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
/// Состояние по умолчанию для умной розетки.
///
impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}
