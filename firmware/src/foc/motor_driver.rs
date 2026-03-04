/// Low-level motor driver interface (PWM output to gate driver).
pub struct MotorDriver {
    _private: (),
}

impl MotorDriver {
    pub fn new() -> Self {
        Self { _private: () }
    }
}
