pub mod calculation;
pub mod physics;
use nalgebra::Vector3 as v3;

use crate::gravity::physics::body::{BodyState, BodyStateBuilder};

pub fn get_planet() -> BodyState {
    BodyStateBuilder::default()
    .r(v3::new(0.0, 0.0, 0.0))
    .v(v3::new(0.0, 0.0, 0.0))
    .build().unwrap()
}
