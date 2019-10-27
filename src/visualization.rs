use crate::player::Player;
use crate::recording::Recording;
use kiss3d::camera::FirstPerson;
use kiss3d::window::Window;
use nalgebra::Point3;
use std::time::{Duration, SystemTime};

pub struct Visualization {
    player: Player,
}

impl Visualization {
    pub fn of(mut recording: Recording, window: &mut Window) {
        let mut player = Player::of(recording);
        player.connectTo(window);
        let mut now = SystemTime::now();
        let time_per_frame = Duration::from_secs_f32(1.0 / 60.0);
        let eye = Point3::new(0.0, 0.0, 0.0);
        let at = Point3::new(-10.0, -10.0, -10.0);
        let mut camera = FirstPerson::new(at, eye);
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
}
