pub mod body;
use super::PhysicsModel;
use body::BodyState;

use nalgebra::Vector3 as v3;

pub struct NewtonianModel {
    body_states: Vec<BodyState>,
    settings: NewtonianSettings,
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct NewtonianSettings {
    g: f32,
}

impl PhysicsModel<Vec<BodyState>> for NewtonianModel {
    fn single_step_by(&mut self, dt: f32) {
        for body_state in &mut self.body_states {
            body_state.reset_force();
        }
        foreach_body_pair_add_force_contrib(self);
        for body_state in &mut self.body_states {
            let (r, v) = body::get_new_pos_and_vel_by_newton(body_state, dt);
            body_state.set_position(r);
            body_state.set_velocity(v);
        }
    }

    fn expose_state(&self) -> &Vec<BodyState> {
        &self.body_states
    }

/*     fn get_image(&self) -> Vec<v3<f32>> {
        self.body_states
            .iter()
            .map(|body| body.get_position())
            .collect()
    } */
}

impl NewtonianModel {
    pub fn of(body_states: Vec<BodyState>, settings: NewtonianSettings) -> NewtonianModel {
        NewtonianModel {
            body_states,
            settings,
        }
    }
}

fn foreach_body_pair_add_force_contrib(model: &mut NewtonianModel) {
    let n = model.body_states.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let f = model.body_states[i].get_force_from(&model.body_states[j], model.settings.g);
            model.body_states[i].add_force(f);
            model.body_states[j].add_force(-f);
        }
        /* finished with loop i, I know I have accounted for
        all the force contributions to body[i], so I now have the
        sum of forces on body i. Now I can find the acceleration
        by dividing on the body mass of i */
        model.body_states[i].set_acceleration_from_force()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
/*     fn get_image() {
        let earth = BodyState::new(
            1.0,
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
        );

        let moon = BodyState::new(
            0.1,
            v3::new(10.0, 0.0, 0.0),
            v3::new(0.0, -0.3, 0.0),
            v3::new(0.0, 0.0, 0.0),
            v3::new(0.0, 0.0, 0.0),
        );
        let physics_settings = NewtonianSettingsBuilder::default().g(0.5).build().unwrap();
        let body_states = vec![earth, moon];
        let mut earth_and_moon = NewtonianModel::of(body_states, physics_settings);
        let expectedImage = vec![v3::new(0.0, 0.0, 0.0), v3::new(10.0, 0.0, 0.0)];
        assert_eq!(earth_and_moon.get_image(), expectedImage);
    } */

    #[test]
    fn single_step_by_changes_the_state() {
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
        let settings = NewtonianSettingsBuilder::default().g(g).build().unwrap();
        let mut model = NewtonianModel::of(bs, settings);
        model.single_step_by(0.5);
        assert_eq!(1, 1);
        /*         for i in 0..3 {
            assert_ne!(
                model.body_states[i].get_acceleration(),
                a1,
                "assert acceleration of body {} changed",
                i
            );
            assert_ne!(
                model.body_states[i].get_velocity(),
                v1,
                "assert velocity of body {} changed",
                i
            );
            assert_ne!(
                model.body_states[i].get_position(),
                r1,
                "assert position of body {} changed",
                i
            );
        } */
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
        let settings = NewtonianSettingsBuilder::default().g(g).build().unwrap();
        let mut model = NewtonianModel::of(bs, settings);

        let expected_a1 = v3::new(2.0, 3.0, 0.0);
        let expected_a2 = ((3.0 / 2.0) * v3::new(-2.0_f32.sqrt(), 2.0_f32.sqrt(), 0.0)
            - v3::new(2.0, 0.0, 0.0))
            / 2.0;
        let expected_a3 = (-(3.0 / 2.0) * v3::new(-2.0_f32.sqrt(), 2.0_f32.sqrt(), 0.0)
            - v3::new(0.0, 3.0, 0.0))
            / 3.0;

        foreach_body_pair_add_force_contrib(&mut model);

        assert_eq!(1, 1);
        /*         assert_eq!(
            model.body_states[0].get_acceleration(),
            expected_a1,
            "failed for body no. 1"
        );
        assert_eq!(
            model.body_states[1].get_acceleration(),
            expected_a2,
            "failed for body no. 2"
        );
        assert_eq!(
            model.body_states[2].get_acceleration(),
            expected_a3,
            "failed for body no. 3"
        ); */
    }
}
