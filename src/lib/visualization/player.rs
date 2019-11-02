use super::super::recording::Recording;
use super::super::recording::SimpleRecording;
use super::super::visualization::Entity;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

pub struct Player {
    number_of_images: usize,
    // nodes: Vec<SceneNode>,
    nodes: Vec<Box<dyn Entity>>,
    current_frame: usize,
}

pub struct PlayerSettings<'a> {
    window: &'a mut Window,
}

impl Player {
    pub fn of(recording: impl Recording, window: &mut Window) -> Player {
        /*         println!(
            "I got a recording of {} frames",
            recording.get_images().len()
        ); */
        let mut player = Player {
            number_of_images: recording.get_images().len(),
            nodes: recording.build_entities(window),
            current_frame: 0,
        };
        // player.connect_to(settings.window);
        player
    }

    pub fn update(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.number_of_images;
        for entity in &mut self.nodes {
            entity.update();
        }
    }
}

impl<'a> PlayerSettings<'a> {
    pub fn of(window: &'a mut Window) -> PlayerSettings {
        PlayerSettings { window }
    }
}
