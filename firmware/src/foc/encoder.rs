/// Rotary encoder interface for position and velocity sensing.
pub struct Encoder {
    _private: (),
}

impl Encoder {
    pub fn new() -> Self {
        Self { _private: () }
    }
}
