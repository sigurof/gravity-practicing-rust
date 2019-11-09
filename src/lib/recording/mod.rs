mod simple_recording;
use super::physics::simulation::Simulation;
use crate::lib::visualization::GameObject;
use kiss3d::window::Window;
use nalgebra::Vector3 as v3;
pub use simple_recording::SimpleRecording;

pub trait Recorder<State, Image> {
    fn time_evolution_per_body(&self) -> Vec<Image>;

    fn record_images_while_simulating(
        &self,
        mut simulation: Simulation<State>,
        settings: RecordingSettings,
    ) -> Vec<Image> {
        let mut images: Vec<Image> = vec![];
        for _ in 0..(settings.number_of_frames) {
            simulation.do_steps(settings.steps_per_frame);
            let image = self.state_to_image(simulation.expose_state());
            images.push(image);
        }
        images
    }

    fn state_to_image(&self, state: &State) -> Image;
}

#[derive(Builder, Default)]
#[builder(setter(into))]
pub struct RecordingSettings {
    steps_per_frame: usize,
    number_of_frames: usize,
}

impl RecordingSettings {
    pub fn default() -> RecordingSettingsBuilder {
        RecordingSettingsBuilder::default()
    }
}
