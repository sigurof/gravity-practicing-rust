use super::super::PointMass;
use super::PhysicsModel;

use nalgebra::Vector3 as v3;

#[derive(Builder)]
#[builder(setter(into))]
pub struct BodyState2 {
    m: f32,
    r: v3<f32>,
    v: v3<f32>,
    a: v3<f32>,
}

impl BodyState2Builder {
    pub fn default() -> BodyState2Builder {
        BodyState2Builder {
            m: Some(0.0),
            r: Some(v3::new(0.0, 0.0, 0.0)),
            v: Some(v3::new(0.0, 0.0, 0.0)),
            a: Some(v3::new(0.0, 0.0, 0.0)),
        }
    }
}

impl BodyState2 {
    pub fn default() -> BodyState2Builder {
        BodyState2Builder::default()
    }
    fn from_point_mass(point_mass: &PointMass) -> BodyState2 {
        BodyState2 {
            m: point_mass.m,
            r: point_mass.r,
            v: point_mass.v,
            a: v3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_mass(&self) -> f32 {
        self.m
    }

    pub fn get_position(&self) -> v3<f32> {
        self.r
    }

    pub fn get_velocity(&self) -> v3<f32> {
        self.v
    }

    pub fn get_acceleration(&self) -> v3<f32> {
        self.a
    }
}

struct UniqueIndexPairs {
    i: usize,
    j: usize,
    max: usize,
}

impl UniqueIndexPairs {
    fn up_to(max: usize) -> UniqueIndexPairs {
        UniqueIndexPairs { i: 0, j: 1, max }
    }
}

impl Iterator for UniqueIndexPairs {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        let j = self.j;

        if self.i == self.max {
            return None;
        }
        if self.j == self.max {
            self.i += 1;
            self.j = self.i + 1;
        } else {
            self.j += 1;
        }
        return Some((i, j));
    }
}

pub struct NewtonianModel2 {
    bodies: Vec<BodyState2>,
    settings: NewtonianSettings2,
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct NewtonianSettings2 {
    g: f32,
}

fn force(on: &BodyState2, from: &BodyState2, g: f32) -> v3<f32> {
    let d12 = from.r - on.r;
    let d = d12.normalize() / d12.norm_squared();
    g * on.m * from.m * d
}

fn newtonian_step(b: &BodyState2, dt: f32) -> (v3<f32>, v3<f32>) {
    let r = b.r + b.v * dt + 0.5 * b.a * dt * dt;
    let v = b.v + b.a * dt;
    (r, v)
}

impl PhysicsModel<Vec<BodyState2>> for NewtonianModel2 {
    fn single_step_by(&mut self, dt: f32) {
        self.zero_out_forces();
        for (i, j) in UniqueIndexPairs::up_to(self.bodies.len() - 1) {
            let f = force(&self.bodies[i], &self.bodies[j], self.settings.g);
            self.bodies[i].a += f;
            self.bodies[j].a -= f;
        }
        for body in &mut self.bodies {
            body.a /= body.m;
            let (r, v) = newtonian_step(body, dt);
            body.r = r;
            body.v = v;
        }
    }

    fn expose_state(&self) -> &Vec<BodyState2> {
        &self.bodies
    }
}

impl NewtonianModel2 {
    pub fn of(point_masses: &Vec<PointMass>, settings: NewtonianSettings2) -> NewtonianModel2 {
        let bodies = point_masses
            .iter()
            .map(|pm| BodyState2::from_point_mass(pm))
            .collect();
        NewtonianModel2 { bodies, settings }
    }

    fn zero_out_forces(&mut self) {
        for body in &mut self.bodies {
            body.a = v3::new(0.0, 0.0, 0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_pairs() {
        let expected_pairs = [(0, 1), (0, 2), (1, 2)];
        let mut idx = 0;
        let mut unique_pairs = UniqueIndexPairs::up_to(2);
        assert_eq!(unique_pairs.next(), Some(expected_pairs[0]));
        assert_eq!(unique_pairs.next(), Some(expected_pairs[1]));
        assert_eq!(unique_pairs.next(), Some(expected_pairs[2]));
    }
}
