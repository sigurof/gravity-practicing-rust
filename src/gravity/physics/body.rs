use nalgebra::Vector3 as v3;

#[derive(Builder)]
#[builder(setter(into))]
pub struct BodyState {
    m: f32,     // mass
    r: v3<f32>, // position
    v: v3<f32>, // velocity
    a: v3<f32>, // acceleration
    f: v3<f32>, // force
}

impl BodyState {
    pub fn new(m: f32, r: v3<f32>, v: v3<f32>, a: v3<f32>, f: v3<f32>) -> BodyState {
        BodyState { m, r, v, a, f }
    }
    pub fn reset_force(&mut self) {
        self.f = v3::new(0.0, 0.0, 0.0);
    }
    pub fn set_acceleration_from_force(&mut self) {
        self.a = self.f / self.m;
    }
    pub fn add_force(&mut self, f: v3<f32>) {
        self.f += f;
    }
    pub fn set_velocity(&mut self, v: v3<f32>) {
        self.v = v;
    }
    pub fn set_position(&mut self, r: v3<f32>) {
        self.r = r;
    }
    pub fn get_position(&self) -> v3<f32> {
        self.r
    }
    pub fn get_force_from(&self, other: &BodyState, g: f32) -> v3<f32> {
        // F = G*m1*m2*(r2-r1) / ||r2-r1||³
        // d12 = (r2 - r1)
        // d = d12 / ||d12||³
        let d12 = other.r - self.r;
        let d = d12.normalize() / d12.norm_squared();
        g * self.m * other.m * d
    }
}

pub fn get_new_pos_and_vel_by_newton(b: &BodyState, dt: f32) -> (v3<f32>, v3<f32>) {
    let r = b.r + b.v * dt + 0.5 * b.a * dt * dt;
    let v = b.v + b.a * dt;
    (r, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_new_pos_and_vel_by_newton_simple_case() {
        let m = 1.0;
        let r = v3::new(0.0, 0.0, 0.0);
        let v = v3::new(1.0, 1.0, 1.0);
        let a = v3::new(0.0, 0.0, 0.0);
        let force = v3::new(0.0, 0.0, 0.0);
        let b = BodyState::new(m, r, v, a, force);
        let dt = 13.0;

        let expected_new_pos = v3::new(13.0, 13.0, 13.0);
        let expected_new_vel = v3::new(1.0, 1.0, 1.0);

        let (new_pos, new_vel) = get_new_pos_and_vel_by_newton(&b, dt);
        assert_eq!(new_pos, expected_new_pos);
        assert_eq!(new_vel, expected_new_vel);
    }

    #[test]
    fn get_new_pos_and_vel_by_newton_hard_case() {
        let m = 1.0;
        let r = v3::new(0.0, 0.0, -1.0);
        let v = v3::new(1.0, 1.0, 1.0);
        let a = v3::new(1.0, 2.0, 3.0);
        let f = v3::new(0.0, 0.0, 0.0);
        let b = BodyState::new(m, r, v, a, f);
        let dt = 1.0;

        let expected_new_pos = v3::new(1.5, 2.0, 1.5);
        let expected_new_vel = v3::new(2.0, 3.0, 4.0);

        let (new_pos, new_vel) = get_new_pos_and_vel_by_newton(&b, dt);
        assert_eq!(new_pos, expected_new_pos);
        assert_eq!(new_vel, expected_new_vel);
    }

    #[test]
    fn get_force_from() {
        let g = 6.67;
        let m1 = 1.0;
        let r1 = v3::new(0.0, 0.0, 0.0);
        let v1 = v3::new(0.0, 0.0, 0.0);
        let a1 = v3::new(0.0, 0.0, 0.0);
        let f1 = v3::new(0.0, 0.0, 0.0);
        let b1 = BodyState::new(m1, r1, v1, a1, f1);

        let m2 = 2.0;
        let r2 = v3::new(3.0, 0.0, 0.0);
        let v2 = v3::new(0.0, 0.0, 0.0);
        let a2 = v3::new(0.0, 0.0, 0.0);
        let f2 = v3::new(0.0, 0.0, 0.0);
        let b2 = BodyState::new(m2, r2, v2, a2, f2);
        // F = G*m1*m2*normalized(r2- r1) / ||r2 - r1||²
        // F = 6.67 * 1.0 * 2.0 * [1, 0, 0] / 9 = [6.67*2.0 / 9, 0, 0]
        let expected = v3::new(6.67 * 2.0 / 9.0, 0.0, 0.0);
        let force_on_b1 = b1.get_force_from(&b2, g);
        assert_eq!(force_on_b1, expected);
    }

    fn get_all_zeros() -> BodyState {
        let mass = 0.0;
        let pos = v3::new(0.0, 0.0, 0.0);
        let vel = v3::new(0.0, 0.0, 0.0);
        let acc = v3::new(0.0, 0.0, 0.0);
        let force = v3::new(0.0, 0.0, 0.0);
        BodyState {
            m: mass,
            r: pos,
            v: vel,
            a: acc,
            f: force,
        }
    }

    #[test]
    fn bs_new() {
        let mass = 1.0;
        let pos = v3::new(0.0, 0.0, 0.0);
        let vel = v3::new(0.0, 0.0, 0.0);
        let acc = v3::new(0.0, 0.0, 0.0);
        let force = v3::new(0.0, 0.0, 0.0);
        let new_body_state = BodyState::new(mass, pos, vel, acc, force);
        let expected = BodyState {
            m: mass,
            r: pos,
            v: vel,
            a: acc,
            f: force,
        };
        assert_eq!(new_body_state.m, expected.m);
        assert_eq!(new_body_state.r, expected.r);
        assert_eq!(new_body_state.v, expected.v);
        assert_eq!(new_body_state.a, expected.a);
    }

    #[test]
    fn bs_set_velocity() {
        let vel = v3::new(1.0, 2.0, 3.0);
        let mut allzero = get_all_zeros();
        allzero.set_velocity(vel);
        assert_eq!(allzero.v, vel);
    }

    #[test]
    fn bs_set_position() {
        let pos = v3::new(1.0, 2.0, 3.0);
        let mut allzero = get_all_zeros();
        allzero.set_position(pos);
        assert_eq!(allzero.r, pos);
    }
}
