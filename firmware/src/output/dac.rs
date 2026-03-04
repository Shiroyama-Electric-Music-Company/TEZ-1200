/// DAC output driver for CV/gate output generation.
pub struct DacOutput {
    _private: (),
}

impl DacOutput {
    pub fn new() -> Self {
        Self { _private: () }
    }
}
