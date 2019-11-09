use super::super::physics::model::{BodyState2, NewtonianModel2, NewtonianSettings2Builder};
use super::super::physics::simulation::Simulation;
use super::super::physics::PointMass;
use super::super::visualization::{GameObject, RecordedGameObject};
use super::{Recorder, RecordingSettings};

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

impl SimpleRecording {
    pub fn new() -> SimpleRecording {
        SimpleRecording {
            images: vec![vec![]],
        }
    }
    pub fn record(&mut self, simulation: Simulation<Vec<BodyState2>>, settings: RecordingSettings) {
        self.images = self.record_images_while_simulating(simulation, settings);
    }
}

impl Recorder<Vec<BodyState2>, Vec<v3<f32>>> for SimpleRecording {
    fn state_to_image(&self, state: &Vec<BodyState2>) -> Vec<v3<f32>> {
        state.iter().map(|bs2| bs2.get_position()).collect()
    }

    fn time_evolution_per_body(&self) -> Vec<Vec<v3<f32>>> {
        let mut slices: Vec<Vec<v3<f32>>> = vec![];
        for obj_idx in 0..self.images[0].len() {
            let mut slice: Vec<v3<f32>> = vec![];
            for time_idx in 0..self.images.len() {
                slice.push(self.images[time_idx][obj_idx]);
            }
            slices.push(slice);
        }
        slices
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
        let steps_per_frame: usize = 10;
        let number_of_frames: usize = 101;
        let settings = RecordingSettings::default()
            .steps_per_frame(steps_per_frame)
            .number_of_frames(number_of_frames)
            .build()
            .unwrap();
        assert_eq!(settings.steps_per_frame, steps_per_frame);
    }

    #[test]
    fn recording_while_simulating() {
        let earth = PointMass::default()
            .m(1.0)
            .r(v3::new(0.0, 0.0, 0.0))
            .v(v3::new(0.0, 0.0, 0.0))
            .build()
            .unwrap();

        let moon = PointMass::default()
            .m(0.1)
            .r(v3::new(0.0, -0.3, 0.0))
            .v(v3::new(0.0, 0.0, 0.0))
            .build()
            .unwrap();
        let body_states = vec![earth, moon];
        let steps_per_frame: usize = 1;
        let number_of_frames: usize = 5;
        let recording_settings = RecordingSettings::default()
            .steps_per_frame(steps_per_frame)
            .number_of_frames(number_of_frames)
            .build()
            .unwrap();
        let simulation_settings = SimulationSettingsBuilder::default()
            .dt(1.0)
            .build()
            .unwrap();
        let g = 600.67;
        let physics_settings = NewtonianSettings2Builder::default().g(g).build().unwrap();
        let body_states_len = body_states.len();
        let earth_and_moon = NewtonianModel2::of(&body_states, physics_settings);
        let simulation = Simulation::of(earth_and_moon, simulation_settings);
        let recorder = SimpleRecording::new();
        let images = recorder.record_images_while_simulating(simulation, recording_settings);
        assert_eq!(images.len(), number_of_frames);
        for image in images {
            assert_eq!(image.len(), body_states_len, "");
        }
    }
}

/*     fn slices(&self) -> Iter<Vec<v3<f32>>> {
    let mut time_slices = vec![];
    for obj_idx in 0..self.images[0].len() {
        let mut positions = vec![];
        for time_idx in 0..self.images.len() {
            positions.push(self.images[time_idx][0]);
        }
        time_slices.push(positions);
    }
    time_slices.iter()
} */

/*     fn build_entities(self, window: &mut Window) -> Vec<Box<dyn Entity>> {
    let mut nodes: Vec<Box<dyn Entity>> = vec![];
    for node_idx in 0..self.get_images()[0].len() {
        let t = Translation3::from(self.get_images()[0][node_idx]);
        let mut obj = window.add_sphere(1.0);
        obj.set_local_translation(t);
        let entity = RecordedGameObject::of(
            self.get_images()
                .into_iter()
                .map(|positions| positions[node_idx])
                .collect(),
            obj,
        );
        nodes.push(Box::from(entity));
    }
    nodes
} */

/* impl<'a> RecordingIter<'a> {
    fn new(images: &'a Vec<Vec<v3<f32>>>) -> RecordingIter<'a> {
        RecordingIter {
            max_idx: images[0].len(),
            obj_idx: 0,
            images,
        }
    }
}

impl<'a> Iterator for RecordingIter<'a> {
    type Item = Vec<v3<f32>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.obj_idx > self.max_idx - 1 {
            return None;
        }
        let positions = vec![];
        for time_idx in 0..self.images.len() {
            positions.push(self.images[time_idx][self.obj_idx]);
        }
        self.obj_idx += 1;
        return Some(positions);
    }
} */
