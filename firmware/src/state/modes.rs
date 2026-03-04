/// Tempo/clock synchronization mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    Internal,
    ExternalClock,
    CvFollow,
    Free,
}

/// Haptic motor behavior mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorMode {
    Locked,
    Assisted,
    Damped,
    SpringReturn,
}

/// Output value quantization mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputQuantization {
    Continuous,
    BeatSynced,
    Chromatic,
    ProgrammableLut,
}

/// Runtime performance state of the controller.
#[derive(Debug, Clone)]
pub struct PerformanceState {
    pub sync_mode: SyncMode,
    pub motor_mode: MotorMode,
    pub quantization: OutputQuantization,
    pub bpm: f32,
    pub current_preset: u8,
}

impl Default for PerformanceState {
    fn default() -> Self {
        Self {
            sync_mode: SyncMode::Internal,
            motor_mode: MotorMode::Locked,
            quantization: OutputQuantization::Continuous,
            bpm: 120.0,
            current_preset: 0,
        }
    }
}
