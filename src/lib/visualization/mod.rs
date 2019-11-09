mod game_object;
mod game_object_with_recording;
use super::recording::Recorder;
pub use {game_object::GameObject, game_object_with_recording::RecordedGameObject};

use kiss3d::camera::{Camera, FirstPerson};
use kiss3d::event::Key;
use kiss3d::renderer::LineRenderer;
use kiss3d::window::Window;
use nalgebra::Point3;
use std::time::{Duration, SystemTime};

pub struct Visualization {
    game_objects: Vec<Box<dyn GameObject>>,
}

impl Visualization {
    pub fn new() -> Visualization {
        Visualization {
            game_objects: vec![],
        }
    }

    pub fn add_game_object(&mut self, game_object: Box<dyn GameObject>) {
        self.game_objects.push(game_object);
    }

    pub fn play(&mut self, mut window: Window) {
        let a = Point3::new(-0.1, -0.1, 0.0);
        let b = Point3::new(0.0, 0.1, 0.0);
        let c = Point3::new(0.1, -0.1, 0.0);

        let mut now = SystemTime::now();
        let time_per_frame = Duration::from_secs_f32(1.0 / 60.0);
        let mut camera: FirstPerson = utils::get_camera();
        while window.render_with_camera(&mut camera) {
            window.draw_line(&a, &b, &Point3::new(-10.0, 0.0, 0.0));
            window.draw_line(&b, &c, &Point3::new(0.0, 11.0, 0.0));
            window.draw_line(&c, &a, &Point3::new(0.0, 0.0, 10.0));
            match now.elapsed() {
                Ok(time_passed) => {
                    if time_passed > time_per_frame {
                        now = SystemTime::now();
                        self.update_game_objects(&mut window);
                    }
                }
                Err(e) => eprintln!("Noooo"),
            }
        }
    }

    pub fn update_game_objects(&mut self, window: &mut Window) {
        for game_object in &mut self.game_objects {
            game_object.update(window);
        }
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
