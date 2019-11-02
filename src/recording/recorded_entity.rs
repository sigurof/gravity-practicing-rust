use crate::player::Entity;
use kiss3d::scene::SceneNode;
use nalgebra::Vector3 as v3;

pub struct RecordedEntity {
    positions: Vec<v3<f32>>,
    scene_node: SceneNode,
}

impl RecordedEntity {
    pub fn of(positions: Vec<v3<f32>>, scene_node: SceneNode) ->RecordedEntity{
        RecordedEntity {
            positions,
            scene_node,
        }
    }
}

impl Entity for RecordedEntity {
    fn update(&self) {}
}
