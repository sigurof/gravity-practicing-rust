use crate::recording::{Recording, SimpleRecording};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

pub trait Entity {
    fn update(&self);
}

pub struct Player {
    recording: Box<dyn Recording>,
    number_of_images: usize,
    // nodes: Vec<SceneNode>,
    nodes: Vec<Box<dyn Entity>>,
    current_frame: usize,
}

pub struct PlayerSettings<'a> {
    window: &'a mut Window,
}

impl Player {
    pub fn of(recording: impl Recording, settings: PlayerSettings) -> Player {
/*         println!(
            "I got a recording of {} frames",
            recording.get_images().len()
        ); */
        let mut player = Player {
            recording: Box::from(recording),
            number_of_images: recording.get_images().len(),
            nodes: vec![],
            current_frame: 0,
        };
        // player.connect_to(settings.window);
        player
    }

    pub fn update(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.number_of_images;
        // for idx in 0..self.nodes.len() {
        // let pos: v3<f32> = self.recording.get_images()[self.current_frame][idx];
        // let t = Translation3::from(pos);
        // self.nodes[idx].set_local_translation(t);
        // }
        for entity in &self.nodes {
            entity.update();
        }
    }

    pub fn connect_to(&mut self, window: &mut Window) {
        self.recording.build_entities(window);
    }
}

impl<'a> PlayerSettings<'a> {
    pub fn of(window: &'a mut Window) -> PlayerSettings {
        PlayerSettings { window }
    }
}
