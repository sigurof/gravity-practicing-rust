mod newtonian;
pub mod old_newton;
use nalgebra::Vector3 as v3;
pub use newtonian::{
    BodyState2, BodyState2Builder, NewtonianModel2, NewtonianSettings2, NewtonianSettings2Builder,
};

pub trait PhysicsModel<T> {
    fn single_step_by(&mut self, dt: f32);

    fn expose_state(&self) -> &T;
}
