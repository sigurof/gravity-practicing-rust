use crate::lib::util::random::random_v3;
use nalgebra::Vector3 as v3;
use std::f32::consts::PI;

#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct Triangle {
    radius: f32,
    center: v3<f32>,
    colors: TriangleColor,
}

#[derive(Clone, Debug)]
pub enum TriangleColor {
    Mono(v3<f32>),
    ByVertex(v3<v3<f32>>),
}

impl TriangleColor {
    fn get(&self, i: usize) -> v3<f32> {
        match self {
            TriangleColor::Mono(c) => *c,
            TriangleColor::ByVertex(c) => {
                let i = i % 3;
                let c = c.clone();
                let result = c[i];
                result
            }
        }
    }
}

pub fn get_two_triangles() -> Vec<f32> {
    vec![
        Triangle::default()
            .radius(0.2)
            .center(v3::new(-0.5, 0.0, 0.0))
            .finished()
            .get_vertex_attributes(),
        Triangle::default()
            .radius(0.3)
            .center(v3::new(0.5, 0.0, 0.0))
            .finished()
            .get_vertex_attributes(),
    ]
    .into_iter()
    .flat_map(|v| v.into_iter())
    .collect()
}

impl Triangle {
    pub fn default() -> TriangleBuilder {
        TriangleBuilder::default()
    }

    pub fn get_vertex_attributes(&self) -> Vec<f32> {
        let mut points = vec![];
        let d_theta = 2.0 * PI / 3.0;
        let theta0 = -PI / 6.0;
        for i in vec![0.0, 1.0, 2.0] {
            let theta = theta0 + i * d_theta;
            let x1 = f32::cos(theta);
            let x2 = f32::sin(theta);
            let point = self.center + self.radius * v3::new(x1, x2, 0.0);
            points.push(point);
        }
        points
            .iter()
            .enumerate()
            .flat_map(|(idx, v)| {
                let c = self.colors.get(idx);
                // let v = v.clone();
                vec![v.x, v.y, v.z, c.x, c.y, c.z].into_iter()
            })
            .collect()
    }
}

impl TriangleBuilder {
    pub fn default() -> TriangleBuilder {
        TriangleBuilder {
            radius: Some(1.0),
            center: Some(v3::new(0.0, 0.0, 0.0)),
            colors: Some(TriangleColor::Mono(random_v3().normalize())),
        }
    }

    pub fn finished(&self) -> Triangle {
        self.build().unwrap()
    }
}
