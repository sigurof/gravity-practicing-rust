mod simple_recording;
use kiss3d::window::Window;
pub use simple_recording::{SimpleRecording};

pub trait Recording {
    fn build_entities(&self, window: &mut Window);
}

pub struct RecordingSettings {
    steps_per_frame: usize,
    number_of_frames: usize,
}
impl RecordingSettings {
    pub fn new() -> RecordingSettings {
        RecordingSettings {
            steps_per_frame: 0,
            number_of_frames: 0,
        }
    }
    pub fn with_steps_per_frame(self, steps_per_frame: usize) -> RecordingSettings {
        RecordingSettings {
            steps_per_frame,
            ..self
        }
    }
    pub fn with_number_of_frames(self, number_of_frames: usize) -> RecordingSettings {
        RecordingSettings {
            number_of_frames,
            ..self
        }
    }
}