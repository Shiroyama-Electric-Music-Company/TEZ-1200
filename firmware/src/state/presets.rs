use super::modes::PerformanceState;

/// Manages storage and recall of user presets.
pub struct PresetManager {
    _private: (),
}

impl PresetManager {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn load(&self, _index: u8) -> PerformanceState {
        PerformanceState::default()
    }

    pub fn save(&mut self, _index: u8, _state: &PerformanceState) {
        // TODO: persist to flash
    }
}
