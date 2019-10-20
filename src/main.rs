use nalgebra::Vector3 as v3;

mod gravity;

use gravity::{
    calculation::{Calculator, Settings},
    physical::{body::BodyState, system::SystemState},
};

fn main() {
    let earth = BodyState::new(
        1.0,
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
    );

    let moon = BodyState::new(
        0.1,
        v3::new(1.0, 0.0, 0.0),
        v3::new(0.0, -1.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
        v3::new(0.0, 0.0, 0.0),
    );

    let mut calc = Calculator::new();
    let mut terra_luna = SystemState::from(vec![earth, moon]);
    let settings = Settings::new(1.0, 1.0, 0.0, 1000, None);
    calc.simulate_to_end(&mut terra_luna, &settings);

    

}
