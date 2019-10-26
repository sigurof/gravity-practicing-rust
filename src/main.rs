mod gravity;
mod recording;
mod visualization;

#[macro_use]
extern crate derive_builder;
use nalgebra::Vector3 as v3;

use gravity::{
    calculation::{Simulation, SimulationSettings, SimulationSettingsBuilder},
    physics::body::BodyState,
    physics::{PhysicsModel, PhysicsSettings, PhysicsSettingsBuilder},
};
use kiss3d::window::Window;
use recording::{Recording, RecordingSettings};
use visualization::{Player, Visualization};

fn main() {
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
    let recording_settings = RecordingSettings::new()
        .with_steps_per_frame(1)
        .with_number_of_frames(10000);
    let simulation_settings = SimulationSettingsBuilder::default()
        .dt(1.0)
        .build()
        .unwrap();

    let g = 0.667;
    let physics_settings = PhysicsSettingsBuilder::default().g(g).build().unwrap();
    let mut earth_and_moon = PhysicsModel::of(vec![earth, moon], physics_settings);
    let mut simulation = Simulation::of(earth_and_moon, simulation_settings);
    let mut recording = Recording::of(simulation, recording_settings);
    let mut window = Window::new("Kiss3d planets");
    let mut player = Player::of(recording);
    player.connectTo(&mut window);
    Visualization::of(player, &mut window);
}
