mod newtonian;
pub use newtonian::{NewtonianModel, NewtonianSettings, NewtonianSettingsBuilder};

pub trait PhysicsModel {
    fn single_step_by(&mut self, dt: f32);
}
