mod newtonian;
pub mod old_newton;
use nalgebra::Vector3 as v3;
pub use newtonian::{NewtonianModel2, NewtonianSettings2, NewtonianSettings2Builder};

pub trait PhysicsModel {
    fn single_step_by(&mut self, dt: f32);

    fn get_image(&self) -> Vec<v3<f32>>;
}
