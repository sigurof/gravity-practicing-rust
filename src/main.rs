use kiss3d;
use nalgebra::{Point3, Translation3, Vector3 as v3};
mod gravity;
use kiss3d::camera::FirstPerson;
use kiss3d::light::Light;
use kiss3d::window::Window;
use std::time::{Duration, SystemTime};

use gravity::{
    calculation::{Recorder, Settings},
    physical::{body::BodyState, system::SystemState},
};

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

    let n: usize = 10000;
    let g = 600.67;
    let mut terra_luna = SystemState::from(vec![earth, moon], g);
    let steps_per_frame: usize = 10;
    let settings = Settings::new(1.0, 0.01, 0.0, n as u32, None);
    let mut rec = Recorder::of(terra_luna, settings, steps_per_frame);
    rec.record();
    let positions = rec.get_positions();

    let eye = Point3::new(0.0, 0.0, 0.0);
    let at = Point3::new(-10.0, -10.0, -10.0);
    let mut camera = FirstPerson::new(at, eye);
    let mut window = Window::new("Kiss3d planets");
    let mut sph = window.add_sphere(1.0);
    let mut sph2 = window.add_sphere(1.0);
    sph.set_color(1.0, 0.0, 0.25);
    sph2.set_color(0.0, 1.0, 0.25);
    window.set_light(Light::StickToCamera);
    let fps = 25;
    let spf = 1.0 / (fps as f32);
    let mut tr = Translation3::new(0.0, 0.0, 0.0);
    let mut tr2 = Translation3::new(0.0, 0.0, 0.0);
    let last_update = SystemTime::now();
    let mut idx = 0;
    while window.render_with_camera(&mut camera) {
        match last_update.elapsed() {
            Ok(elapsed) => {
                tr = Translation3::from_vector(positions[idx][0]);
                tr2 = Translation3::from_vector(positions[idx][1]);
                sph.set_local_translation(tr);
                sph2.set_local_translation(tr2);
                idx = (idx + 1) % rec.get_n_frames();
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
