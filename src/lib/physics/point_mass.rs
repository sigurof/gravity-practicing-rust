use nalgebra::Vector3 as v3;

#[derive(Builder)]
#[builder(setter(into))]
pub struct PointMass {
    pub m: f32,
    pub r: v3<f32>,
    pub v: v3<f32>,
}

impl PointMassBuilder {
    pub fn default() -> PointMassBuilder {
        PointMassBuilder {
            m: Some(0.0),
            r: Some(v3::new(0.0, 0.0, 0.0)),
            v: Some(v3::new(0.0, 0.0, 0.0)),
        }
    }
}
