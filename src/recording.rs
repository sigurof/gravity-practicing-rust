use crate::gravity::{
    calculation::{Simulation, SimulationSettings, SimulationSettingsBuilder},
    physics::{PhysicsModel, PhysicsSettingsBuilder},
};
use kiss3d::camera::Camera;
use kiss3d::camera::FirstPerson;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Point3;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

pub struct Recording {
    images: Vec<Vec<v3<f32>>>,
}

impl Recording {
    pub fn of(simulation: Simulation, settings: RecordingSettings) -> Recording {
        Recording {
            images: Recording::record_images_while_simulating(simulation, settings),
        }
    }

    fn record_images_while_simulating(
        mut simulation: Simulation,
        settings: RecordingSettings,
    ) -> Vec<Vec<v3<f32>>> {
        let mut images: Vec<Vec<v3<f32>>> =
            vec![simulation.do_steps(settings.steps_per_frame).get_image()];
        for _ in 0..(settings.number_of_frames - 1) {
            images.push(simulation.do_steps(settings.steps_per_frame).get_image());
        }
        images
    }

    pub fn get_images(&self) -> &Vec<Vec<v3<f32>>> {
        &self.images
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gravity::{physics::body::BodyState, physics::PhysicsSettings};

    #[test]
    fn recording_settings_builder_works() {
        let steps_per_frame = 10;
        let number_of_frames = 101;
        let settings = RecordingSettings::new()
            .with_steps_per_frame(steps_per_frame)
            .with_number_of_frames(number_of_frames);
        assert_eq!(settings.steps_per_frame, steps_per_frame);
    }

    #[test]
    fn recording_while_simulating() {
        let earth = BodyState::new(
            1.0,
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
        );

        let moon = BodyState::new(
            0.1,
            v3::new(10.0, 0.0, 0.0),
            v3::new(0.0, -0.3, 0.0),
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
        );
        let body_states = vec![earth, moon];
        let steps_per_frame: usize = 1;
        let number_of_frames: usize = 5;
        let recording_settings = RecordingSettings::new()
            .with_steps_per_frame(steps_per_frame)
            .with_number_of_frames(number_of_frames);
        let simulation_settings = SimulationSettingsBuilder::default()
            .dt(1.0)
            .build()
            .unwrap();
        let g = 600.67;
        let physics_settings = PhysicsSettingsBuilder::default().g(g).build().unwrap();
        let body_states_len = body_states.len();
        let earth_and_moon = PhysicsModel::of(body_states, physics_settings);
        let simulation = Simulation::of(earth_and_moon, simulation_settings);
        let images = Recording::record_images_while_simulating(simulation, recording_settings);
        assert_eq!(images.len(), number_of_frames);
        for image in images {
            assert_eq!(image.len(), body_states_len, "");
        }
    }
}
