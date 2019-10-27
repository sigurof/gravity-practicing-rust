use crate::recording::Recording;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

pub struct Player {
    recording: Recording,
    nodes: Vec<SceneNode>,
    current_frame: usize,
}

impl Player {
    pub fn of(recording: Recording) -> Player {
        println!(
            "I got a recording of {} frames",
            recording.get_images().len()
        );
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
