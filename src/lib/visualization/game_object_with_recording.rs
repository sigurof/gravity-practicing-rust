use super::game_object::GameObject;
use crate::lib::util::random::random_v3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Point3;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

#[derive(Builder)]
#[builder(setter(into))]
pub struct RecordedGameObject {
    current_frame: usize,
    positions: Vec<v3<f32>>,
    scene_node: SceneNode,
    draw_trail: Option<usize>,
    color: Point3<f32>,
}

impl RecordedGameObjectBuilder {
    pub fn default() -> RecordedGameObjectBuilder {
        RecordedGameObjectBuilder {
            current_frame: Some(0),
            positions: Some(vec![]),
            scene_node: None,
            draw_trail: Some(None),
            color: Some(Point3::from(random_v3().normalize())),
        }
    }

    pub fn finished(&self) -> RecordedGameObject {
        self.build().unwrap()
    }
}

impl RecordedGameObject {
    pub fn default() -> RecordedGameObjectBuilder {
        RecordedGameObjectBuilder::default()
    }

    fn next(&mut self, window: &mut Window) {
        self.current_frame = (self.current_frame + 1) % self.positions.len();
        self.scene_node
            .set_local_translation(Translation3::from(self.positions[self.current_frame]));
        if let Some(n_segments) = self.draw_trail {
            // for i in 0..n_segments {
            let b = Point3::from(self.positions[self.current_frame]);
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &b, &self.color);
            // }
        }
    }
}

impl GameObject for RecordedGameObject {
    fn update(&mut self, window: &mut Window) {
        self.next(window);
    }
}
