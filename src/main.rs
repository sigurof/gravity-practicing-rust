mod gravity;
mod player;
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
use player::{Player, PlayerSettings};
use recording::{Recording, RecordingSettings};
use visualization::Visualization;

fn main() {
    let earth = BodyState::new(
        1.0,
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
    );

    let moon = BodyState::new(
        0.01,
        v3::new(10.0, 0.0, 0.0),
        v3::new(0.0, -0.3, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
    );

    let moon2 = BodyState::new(
        0.01,
        v3::new(0.0, 5.0, 0.0),
        v3::new(0.4, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
    );

    let moon3 = BodyState::new(
        0.01,
        v3::new(0.0, -5.0, 0.0),
        v3::new(0.0, 0.0, 0.35),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
    );
    let recording_settings = RecordingSettings::new()
        .with_steps_per_frame(10)
        .with_number_of_frames(20000);
    let simulation_settings = SimulationSettingsBuilder::default()
        .dt(0.01)
        .build()
        .unwrap();
    let mut window = Window::new("Kiss3d planets");
    let playerSettings = PlayerSettings::of(&mut window);
    let g = 0.667;
    let bodies = gravity::get_sun_planet_moon(100.0, 1.0, g);

    let physics_settings = PhysicsSettingsBuilder::default().g(g).build().unwrap();
    let mut two_body_system = PhysicsModel::of(bodies, physics_settings);
    // let mut earth_and_moon = PhysicsModel::of(vec![earth, moon, moon2, moon3], physics_settings);
    // let mut simulation = Simulation::of(earth_and_moon, simulation_settings);
    let mut simulation = Simulation::of(two_body_system, simulation_settings);
    let mut recording = Recording::of(simulation, recording_settings);
    let mut player = Player::of(recording, playerSettings);
    // player.
    Visualization::of(player, &mut window);
}
