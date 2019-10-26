use crate::recording::Recording;
use kiss3d::window::Window;
use nalgebra::Point3;
use nalgebra::Translation3;

use kiss3d::camera::Camera;
use kiss3d::camera::FirstPerson;
use kiss3d::scene::SceneNode;
use nalgebra::Vector3 as v3;
use std::time::{Duration, SystemTime};

pub struct Visualization {
    player: Player,
}

impl Visualization {
    pub fn of(mut player: Player, window: &mut Window) {
        let mut now = SystemTime::now();
        let time_per_frame = Duration::from_secs_f32(1.0 / 60.0);
        let eye = Point3::new(0.0, 0.0, 0.0);
        let at = Point3::new(-10.0, -10.0, -10.0);
        let mut camera = FirstPerson::new(eye, at);
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

pub struct Player {
    recording: Recording,
    nodes: Vec<SceneNode>,
    current_frame: usize,
}

impl Player {
    pub fn of(recording: Recording) -> Player {
        println!("I got a recording of {} frames", recording.get_images().len());
        Player {
            recording,
            nodes: vec![],
            current_frame: 0,
        }
    }

    pub fn connectTo(&mut self, window: &mut Window) {
        for pos in &self.recording.get_images()[0] {
            let t = Translation3::from(*pos);
            let mut obj = window.add_sphere(1.0);
            obj.set_local_translation(t);
            self.nodes.push(obj);
        }
    }

    pub fn update(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.recording.get_images().len();
        for idx in 0..self.nodes.len() {
            let pos: v3<f32> = self.recording.get_images()[self.current_frame][idx];
            let t = Translation3::from(pos);
            self.nodes[idx].set_local_translation(t);
        }
    }
}
