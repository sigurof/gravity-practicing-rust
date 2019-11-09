use super::{PointMass, PointMassBuilder};
// use body::{PointMass, PointMassBuilder};

use crate::lib::util::random::{random_between, random_angle, random_v3};
use nalgebra::Vector3 as v3;
use std::f32::consts::PI;

#[allow(dead_code)]
pub fn get_sun_planet_moon(t1: f32, t2: f32, g: f32) -> Vec<PointMass> {
    let m = 100.0;
    let m2 = 1.0;
    let m21 = 0.1 * m2;
    let m22 = 0.9 * m2;
    let m1 = m - m2;

    get_sun_planet_moon_more_parameters(t1, t2, g, m1, m21, m22)
}

pub fn get_sun_planet_moon_more_parameters(
    t1: f32,
    t2: f32,
    g: f32,
    m1: f32,
    m21: f32,
    m22: f32,
) -> Vec<PointMass> {
    let bary_pos = v3::new(0.0, 0.0, 0.0);
    let bary_vel = v3::new(0.0, 0.0, 0.0);

    let m2: f32 = m21 + m22;
    let (b1, b2) = two_body_system(bary_pos, bary_vel, m1, m2, g, t1);

    let (b21, b22) = two_body_system(b2.r, b2.v, m21, m22, g, t2);
    vec![b1, b21, b22]
}

#[allow(dead_code)]
pub fn get_one_orbiting_two(t12: f32, t2: f32, g: f32) -> Vec<PointMass> {
    let bary_pos = v3::new(0.0, 0.0, 0.0);
    let bary_vel = v3::new(0.0, 0.0, 0.0);
    let m = 100.0;

    let m1: f32 = random_between(60.0, 99.0);
    let m2: f32 = m - m1;
    let (b1, b2) = two_body_system(bary_pos, bary_vel, m1, m2, g, t2);

    let m11 = random_between(0.3 * m1, 0.7 * m1);
    let m12 = m1 - m11;
    let (b11, b12) = two_body_system(b1.r, b1.v, m11, m12, g, t12);
    vec![b11, b12, b2]
}

#[allow(dead_code)]
pub fn poc_create_two_body_system(g: f32) -> (PointMass, PointMass) {
    // All values describe initial values
    // bary = barycenter
    // Choose G
    // 1. Determine barycenter position, velocity and mass1 and mass2
    let bary_pos = v3::new(0.0, 0.0, 0.0);
    let bary_vel = v3::new(0.0, 0.0, 0.0);
    let m1: f32 = random_between(80.0, 100.0);
    let m2: f32 = random_between(1.0, 20.0);
    let t = random_between(1.0, 50.0);
    two_body_system(bary_pos, bary_vel, m1, m2, g, t)
}

pub fn two_body_system(
    bary_pos: v3<f32>,
    bary_vel: v3<f32>,
    m1: f32,
    m2: f32,
    g: f32,
    t: f32,
) -> (PointMass, PointMass) {
    let mu = m1 * m2 / (m1 + m2);
    let (r_vec, v_vec) = get_central_force_problem_position_and_velocity(g, m1, m2, t);

    let r1_vec = bary_pos + mu / m1 * r_vec;
    let v1_vec = bary_vel + mu / m1 * v_vec;

    let r2_vec = bary_pos - mu / m2 * r_vec;
    let v2_vec = bary_vel - mu / m2 * v_vec;

    let b1 = PointMassBuilder::default()
        .m(m1)
        .r(r1_vec)
        .v(v1_vec)
        .build()
        .unwrap();

    let b2 = PointMassBuilder::default()
        .m(m2)
        .r(r2_vec)
        .v(v2_vec)
        .build()
        .unwrap();
    (b1, b2)
}

fn get_central_force_problem_position_and_velocity(
    g: f32,
    m1: f32,
    m2: f32,
    t: f32,
) -> (v3<f32>, v3<f32>) {
    // calculate mu for later:
    let mu: f32 = m1 * m2 / (m1 + m2);
    let m = m1 + m2;

    let r_hat = random_v3().normalize();
    let theta_hat = r_hat.cross(&random_v3()).normalize();
    let e = random_between(0.2, 0.8);
    let theta = random_angle();

    let gamma = g * m1 * m2;

    // let l = mu * f32::powf(G * G * M * M / 2.0 / T, 1.0 / 3.0) * f32::sqrt(1.0 - e * e);
    let l = m1 * m2 * f32::powf(g * g * t / 2.0 / PI / m, 1.0 / 3.0) / f32::sqrt(1.0 - e * e);

    let r_vec = (l * l / mu / gamma) / (1.0 + e * f32::cos(theta)) * r_hat;
    let v_vec =
        (gamma / l) * (e * f32::sin(theta) * r_hat + (1.0 + e * f32::cos(theta)) * theta_hat);
    return (r_vec, v_vec);
}


#[allow(dead_code)]
fn calculate_energy(b1: PointMass, b2: PointMass, g: f32) -> f32 {
    let distance = (b1.r - b2.r).norm();
    let m1 = b1.m;
    let m2 = b2.m;
    let v1 = b1.v;
    let v2 = b2.v;
    0.5 * m1 * v1.norm_squared() + 0.5 * m2 * v2.norm_squared() - g * m1 * m2 / distance
}

#[allow(dead_code)]
fn momentum(b: PointMass) -> v3<f32> {
    b.m * b.v
}

#[allow(dead_code)]
fn potential(g: f32, m1: f32, m2: f32, distance: v3<f32>) -> f32 {
    -g * m1 * m2 / distance.norm()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_is_calculated_correctly() {
        let g = 1.0;
        let m1 = 1.0;
        let r1 = v3::new(-1.50, 0.0, 0.0);
        let v1 = v3::new(0.0, 3.0, 0.0);

        let m2 = 2.0;
        let r2 = v3::new(1.0, 0.0, 0.0);
        let v2 = v3::new(1.0, 0.0, 0.0);

        let expected_energy =
            0.5 * (m1 * v1.norm_squared() + m2 * v2.norm_squared()) + potential(g, m1, m2, r2 - r1);
        let b1 = PointMassBuilder::default()
            .m(m1)
            .r(r1)
            .v(v1)
            .build()
            .unwrap();
        let b2 = PointMassBuilder::default()
            .m(m2)
            .r(r2)
            .v(v2)
            .build()
            .unwrap();
        let energy = calculate_energy(b1, b2, g);
        assert_eq!(energy, expected_energy);
    }

    #[test]
    fn test_random_angle_returns_value_between_0_and_2pi() {
        for _ in 0..100 {
            let angle = random_angle();
            assert!(
                angle < 2.0 * PI,
                "expected angle to be less than 2pi. However, it was {}",
                angle
            );
            assert!(
                angle > 0.0,
                "expected angle to more than 0.0. However, it was {}",
                angle
            );
        }
    }

    #[test]
    fn test_two_bodys_have_energy_below_zero() {
        for _ in 0..100 {
            let g = 10.0;
            let (b1, b2) = poc_create_two_body_system(g);
            let energy = calculate_energy(b1, b2, g);
            assert!(
                energy < 0.0,
                "Expected energy to be less than 0.0. However, it was {}",
                energy
            );
        }
    }
}
