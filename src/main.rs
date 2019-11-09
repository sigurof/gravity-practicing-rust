mod lib;
use lib::physics::gravity;
use lib::physics::model::{NewtonianModel2, NewtonianSettings2Builder};
use lib::physics::simulation::Simulation;
use lib::physics::simulation::SimulationSettingsBuilder;
use lib::recording::RecordingSettings;
use lib::recording::{Recorder, SimpleRecording};
use lib::visualization::{RecordedGameObject, Visualization};
use std::f32::consts::PI;

#[macro_use]
extern crate derive_builder;
use nalgebra::Vector3 as v3;

use kiss3d::window::Window;

fn main() {
    let recording_settings = RecordingSettings::default()
        .steps_per_frame(1 as usize)
        .number_of_frames(20000 as usize)
        .build()
        .unwrap();
    let simulation_settings = SimulationSettingsBuilder::default()
        .dt(0.01)
        .build()
        .unwrap();
    let mut window = Window::new("Kiss3d planets");
    let g = 0.667;
    let density = 20.0;
    let bodies = gravity::get_sun_planet_moon_more_parameters(50.0, 1.0, g, 100.0, 1.0, 0.1);

    let physics_settings = NewtonianSettings2Builder::default().g(g).build().unwrap();
    let mut two_body_system = NewtonianModel2::of(&bodies, physics_settings);
    let simulation = Simulation::of(two_body_system, simulation_settings);
    let mut recording = SimpleRecording::new();
    recording.record(simulation, recording_settings);
    let mut idx: usize = 0;
    let mut v = Visualization::new();
    for positions in recording.time_evolution_per_body() {
        v.add_game_object(Box::from(
            RecordedGameObject::default()
                .draw_trail(Some(5))
                .positions(positions)
                .scene_node(window.add_sphere(f32::powf(
                    3.0 * bodies[idx].m / density / 4.0 / PI,
                    1.0 / 3.0,
                )))
                .finished(), // .build().unwrap()
        ));
        idx += 1;
    }
    v.play(window);
}
