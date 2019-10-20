use crate::gravity::physical::body;
use body::BodyState;
use nalgebra::Vector3 as v3;

pub struct SystemState {
    bodies: Vec<BodyState>,
}

impl SystemState {
    pub fn from(bodies: Vec<BodyState>) -> SystemState {
        SystemState { bodies }
    }
}

fn foreach_body_pair_add_force_contrib(sys: &mut SystemState, g: f32) {
    let n = sys.bodies.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let f = sys.bodies[i].get_force_from(&sys.bodies[j], g);
            // let a_i = f / sys.bodies[i].get_mass();
            // let a_j = -f / sys.bodies[j].get_mass();
            sys.bodies[i].add_force(f);
            sys.bodies[j].add_force(-f);
        }
        sys.bodies[i].set_acceleration_from_force()
    }
}

pub fn step_by_newton(sys: &mut SystemState, dt: f32, g: f32) {
    // TODO: Implement an iterator which can do this:
    // for unique_body_pair in sys.bodies {
    // set_accelerations_by_newtons_gravitational_law(unique_body_pair);
    // }
    for body in &mut sys.bodies {
        body.reset_force();
    }
    foreach_body_pair_add_force_contrib(sys, g);
    for body in &mut sys.bodies {
        let (r, v) = body::get_new_pos_and_vel_by_newton(body, dt);
        body.set_position(r);
        body.set_velocity(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_by_newton_changes_the_state(){
        let dt = 10.0;
        let g = 1.0;
        let m1 = 1.0;
        let r1 = v3::new(0.0, 0.0, 0.0);
        let v1 = v3::new(0.0, 0.0, 0.0);
        let a1 = v3::new(0.0, 0.0, 0.0);
        let f1 = v3::new(0.0, 0.0, 0.0);
        let b1 = BodyState::new(m1, r1, v1, a1, f1);
        let m2 = 2.0;
        let r2 = v3::new(1.0, 0.0, 0.0);
        let v2 = v3::new(0.0, 0.0, 0.0);
        let a2 = v3::new(0.0, 0.0, 0.0);
        let f2 = v3::new(0.0, 0.0, 0.0);
        let b2 = BodyState::new(m2, r2, v2, a2, f2);
        let m3 = 3.0;
        let r3 = v3::new(0.0, 1.0, 0.0);
        let v3 = v3::new(0.0, 0.0, 0.0);
        let a3 = v3::new(0.0, 0.0, 0.0);
        let f3 = v3::new(0.0, 0.0, 0.0);
        let b3 = BodyState::new(m3, r3, v3, a3, f3);
        let bs = vec![b1, b2, b3];
        let mut sys = SystemState::from(bs);
        step_by_newton(&mut sys, dt, g);
        assert_ne!(sys.bodies[0].get_acceleration(), a1, "assert acceleration of body 1 changed");
        assert_ne!(sys.bodies[1].get_acceleration(), a2, "assert acceleration of body 2 changed");
        assert_ne!(sys.bodies[2].get_acceleration(), a3, "assert acceleration of body 3 changed");
        assert_ne!(sys.bodies[0].get_velocity(), v1, "assert velocity of body 1 changed");
        assert_ne!(sys.bodies[1].get_velocity(), v2, "assert velocity of body 2 changed");
        assert_ne!(sys.bodies[2].get_velocity(), v3, "assert velocity of body 3 changed");
        assert_ne!(sys.bodies[0].get_position(), r1, "assert position of body 1 changed");
        assert_ne!(sys.bodies[1].get_position(), r2, "assert position of body 2 changed");
        assert_ne!(sys.bodies[2].get_position(), r3, "assert position of body 3 changed");
    }

    #[test]
    fn test_that_function_sums_force_contributions_correctly() {
        /*
        b1 + b2:
        F = G * m1*m2*norm(d12)/square_norm(d12)
        d12 = r2 - r1 = [1, 0, 0]
        m1 = 1, m2 = 2, G = 1.0
        F = 1.0 * 1.0 * 2.0 * [1, 0, 0] = [2, 0, 0]
        a1 =  [2, 0, 0] / 1.0 =  [2, 0, 0]
        a2 = -[2, 0, 0] / 2.0 = -[1, 0, 0]

        b1 + b3
        d13 = r3 - r1 = [0, 1, 0]
        m1 = 1, m3 = 3, G = 1.0
        F = 1.0 * 1.0 * 3.0 * [0, 1, 0] = [0, 3, 0]
        ____ a1 =  [0, 3, 0] / 1.0 + [2, 0, 0] =  [2, 3, 0] ____
        a3 = -[0, 3, 0] / 3.0             = -[0, 1, 0]

        b2 + b3
        b2 + b3
        d23 = r3 - r2 = [0, 1, 0] - [1, 0, 0] = [-1, 1, 0]
        m2 = 2, m3 = 3, G = 1.0
        F = 1.0 * 2.0 * 3.0 * [-sqrt(2)/2, sqrt(2)/2, 0] / 2 = 3/2*[-sqrt(2), sqrt(2), 0]
        ____ a2 =  3/2*[-sqrt(2), sqrt(2), 0] / 2.0 - [1, 0, 0] =  3/4*[-sqrt(2), sqrt(2), 0] - [1, 0, 0]____
        ____ a3 = -3/2*[-sqrt(2), sqrt(2), 0] / 3.0 - [0, 1, 0] = -1/2*[-sqrt(2), sqrt(2), 0] - [0, 1, 0]____
         */
        let g = 1.0;
        let m1 = 1.0;
        let r1 = v3::new(0.0, 0.0, 0.0);
        let v1 = v3::new(0.0, 0.0, 0.0);
        let a1 = v3::new(0.0, 0.0, 0.0);
        let f1 = v3::new(0.0, 0.0, 0.0);
        let b1 = BodyState::new(m1, r1, v1, a1, f1);

        let m2 = 2.0;
        let r2 = v3::new(1.0, 0.0, 0.0);
        let v2 = v3::new(0.0, 0.0, 0.0);
        let a2 = v3::new(0.0, 0.0, 0.0);
        let f2 = v3::new(0.0, 0.0, 0.0);
        let b2 = BodyState::new(m2, r2, v2, a2, f2);

        let m3 = 3.0;
        let r3 = v3::new(0.0, 1.0, 0.0);
        let v3 = v3::new(0.0, 0.0, 0.0);
        let a3 = v3::new(0.0, 0.0, 0.0);
        let f3 = v3::new(0.0, 0.0, 0.0);
        let b3 = BodyState::new(m3, r3, v3, a3, f3);
        let bs = vec![b1, b2, b3];
        let mut sys = SystemState::from(bs);

        let expected_a1 = v3::new(2.0, 3.0, 0.0);
        let expected_a2 = ((3.0 / 2.0) * v3::new(-2.0_f32.sqrt(), 2.0_f32.sqrt(), 0.0)
            - v3::new(2.0, 0.0, 0.0))
            / 2.0;
        let expected_a3 = (-(3.0 / 2.0) * v3::new(-2.0_f32.sqrt(), 2.0_f32.sqrt(), 0.0)
            - v3::new(0.0, 3.0, 0.0))
            / 3.0;

        foreach_body_pair_add_force_contrib(&mut sys, g);

        assert_eq!(
            sys.bodies[0].get_acceleration(),
            expected_a1,
            "failed for body no. 1"
        );
        assert_eq!(
            sys.bodies[1].get_acceleration(),
            expected_a2,
            "failed for body no. 2"
        );
        assert_eq!(
            sys.bodies[2].get_acceleration(),
            expected_a3,
            "failed for body no. 3"
        );
    }
}
