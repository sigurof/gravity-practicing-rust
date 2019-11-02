use super::Entity;
use kiss3d::scene::SceneNode;
use nalgebra::Translation3;
use nalgebra::Vector3 as v3;

pub struct EntityWithRecording {
    current_frame: usize,
    positions: Vec<v3<f32>>,
    scene_node: SceneNode,
}

impl EntityWithRecording {
    pub fn of(positions: Vec<v3<f32>>, scene_node: SceneNode) -> EntityWithRecording {
        EntityWithRecording {
            current_frame: 0,
            positions,
            scene_node,
        }
    }

    fn next(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.positions.len();
        self.scene_node
            .set_local_translation(Translation3::from(self.positions[self.current_frame]));
    }
}

impl Entity for EntityWithRecording {
    fn update(&mut self) {
        self.next();
    }
}
