use super::super::physics::simulation::Simulation;
use super::super::visualization::{Entity, EntityWithRecording};
use super::{Recording, RecordingSettings};

use kiss3d::camera::Camera;
use kiss3d::camera::FirstPerson;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Point3;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

pub struct SimpleRecording {
    images: Vec<Vec<v3<f32>>>,
}

impl Recording for SimpleRecording {
    fn get_images(&self) -> &Vec<Vec<v3<f32>>> {
        &self.images
    }
    fn build_entities(self, window: &mut Window) -> Vec<Box<dyn Entity>> {
        let mut nodes: Vec<Box<dyn Entity>> = vec![];
        for node_idx in 0..self.get_images()[0].len() {
            let t = Translation3::from(self.get_images()[0][node_idx]);
            let mut obj = window.add_sphere(1.0);
            obj.set_local_translation(t);
            let entity = EntityWithRecording::of(
                self.get_images()
                    .into_iter()
                    .map(|positions| positions[node_idx])
                    .collect(),
                obj,
            );
            nodes.push(Box::from(entity));
        }
        nodes
    }
}

impl SimpleRecording {
    pub fn of(simulation: Simulation, settings: RecordingSettings) -> SimpleRecording {
        SimpleRecording {
            images: SimpleRecording::record_images_while_simulating(simulation, settings),
        }
    }

    fn record_images_while_simulating(
        mut simulation: Simulation,
        settings: RecordingSettings,
    ) -> Vec<Vec<v3<f32>>> {
        let mut images: Vec<Vec<v3<f32>>> = vec![];
        for _ in 0..(settings.number_of_frames) {
            simulation.do_steps(settings.steps_per_frame);
            images.push(simulation.get_physics_model().get_image());
        }
        images
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::physics::{
        model::old_newton::body::BodyState,
        model::old_newton::{NewtonianModel, NewtonianSettingsBuilder},
        simulation::{SimulationSettings, SimulationSettingsBuilder},
    };
    use super::*;

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
        let physics_settings = NewtonianSettingsBuilder::default().g(g).build().unwrap();
        let body_states_len = body_states.len();
        let earth_and_moon = NewtonianModel::of(body_states, physics_settings);
        let simulation = Simulation::of(earth_and_moon, simulation_settings);
        let images =
            SimpleRecording::record_images_while_simulating(simulation, recording_settings);
        assert_eq!(images.len(), number_of_frames);
        for image in images {
            assert_eq!(image.len(), body_states_len, "");
        }
    }
}
