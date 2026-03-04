/// Sync clock manager for internal/external tempo synchronization.
pub struct SyncClock {
    _private: (),
}

impl SyncClock {
    pub fn new() -> Self {
        Self { _private: () }
    }
}
