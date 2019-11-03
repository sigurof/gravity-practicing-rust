mod lib;
use lib::physics::gravity;
use lib::physics::gravity::body::BodyState;
use lib::physics::model::{NewtonianModel, NewtonianSettingsBuilder};
use lib::physics::simulation::Simulation;
use lib::physics::simulation::SimulationSettingsBuilder;
use lib::recording::RecordingSettings;
use lib::recording::SimpleRecording;
use lib::visualization::Player;
use lib::visualization::Visualization;

#[macro_use]
extern crate derive_builder;
use nalgebra::Vector3 as v3;

use kiss3d::window::Window;

fn main() {
    let recording_settings = RecordingSettings::new()
        .with_steps_per_frame(10)
        .with_number_of_frames(20000);
    let simulation_settings = SimulationSettingsBuilder::default()
        .dt(0.01)
        .build()
        .unwrap();
    let mut window = Window::new("Kiss3d planets");
    // let playerSettings = PlayerSettings::of(&mut window);
    let g = 0.667;
    let bodies = gravity::get_sun_planet_moon(100.0, 1.0, g);

    let physics_settings = NewtonianSettingsBuilder::default().g(g).build().unwrap();
    let mut two_body_system = NewtonianModel::of(bodies, physics_settings);
    // let mut earth_and_moon = NewtonianModel::of(vec![earth, moon, moon2, moon3], physics_settings);
    // let mut simulation = Simulation::of(earth_and_moon, simulation_settings);
    let mut simulation = Simulation::of(two_body_system, simulation_settings);
    let mut recording = SimpleRecording::of(simulation, recording_settings);
    let mut player = Player::of(recording, &mut window);
    // player.
    Visualization::of(player, &mut window);
}
