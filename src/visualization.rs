use crate::player::{Player, PlayerSettings};
use crate::recording::Recording;
use kiss3d::camera::{ArcBall, Camera, FirstPerson};
use kiss3d::event::Key;
use kiss3d::window::Window;
use nalgebra::Point3;
use std::time::{Duration, SystemTime};

pub struct Visualization {}

impl Visualization {
    pub fn of(mut player: Player, window: &mut Window) {
        let mut now = SystemTime::now();
        let time_per_frame = Duration::from_secs_f32(1.0 / 60.0);
        let mut camera: FirstPerson = utils::get_camera();
        while window.render_with_camera(&mut camera) {
            match now.elapsed() {
                Ok(time_passed) => {
                    if time_passed > time_per_frame {
                        now = SystemTime::now();
                        player.update();
                    }
                }
                Err(e) => eprintln!("Noooo"),
            }
        }
    }

    pub fn from_recording(recording: Recording, window: &mut Window) {
        let playerSettings = PlayerSettings::of(window);
        let mut player = Player::of(recording, playerSettings);
        Visualization::of(player, window);
    }
}

mod utils {
    use super::{Camera, FirstPerson, Key, Point3};

    pub fn get_camera() -> FirstPerson {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let at = Point3::new(-30.0, -30.0, -30.0);
        let mut camera = FirstPerson::new(at, eye);
        camera.rebind_up_key(Some(Key::W));
        camera.rebind_down_key(Some(Key::S));
        camera.rebind_left_key(Some(Key::A));
        camera.rebind_right_key(Some(Key::D));
        camera
    }
}
