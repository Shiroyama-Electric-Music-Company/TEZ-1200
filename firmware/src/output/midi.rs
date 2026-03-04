/// MIDI output interface for note/CC message generation.
pub struct MidiOutput {
    _private: (),
}

impl MidiOutput {
    pub fn new() -> Self {
        Self { _private: () }
    }
}
