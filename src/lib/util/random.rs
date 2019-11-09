use nalgebra::Vector3 as v3;
use rand::thread_rng as rng;
use rand::Rng;
use std::f32::consts::PI;

pub fn random_between(min: f32, max: f32) -> f32 {
    let between_0_and_1: f32 = rng().gen();
    min + (max - min) * between_0_and_1
}

pub fn random_angle() -> f32 {
    let between_0_and_1: f32 = rng().gen();
    between_0_and_1 * 2.0 * PI
}

pub fn random_v3() -> v3<f32> {
    let x = rng().gen();
    let y = rng().gen();
    let z = rng().gen();
    v3::new(x, y, z)
}
