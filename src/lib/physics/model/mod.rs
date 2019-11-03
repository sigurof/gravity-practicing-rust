mod newtonian;
use nalgebra::Vector3 as v3;
pub use newtonian::{NewtonianModel, NewtonianSettings, NewtonianSettingsBuilder};

pub trait PhysicsModel {
    fn single_step_by(&mut self, dt: f32);

    fn get_image(&self) -> Vec<v3<f32>>;
}
