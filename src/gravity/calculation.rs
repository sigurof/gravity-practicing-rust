use crate::gravity::physical::system;
use system::SystemState;

pub struct Runner {
    calc: Calculator,
}


pub struct Calculator {
    present_time: f32,
    present_step: u32,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            present_step: 0,
            present_time: 0.0,
        }
    }

    fn iterate(&mut self, system: &mut SystemState, settings: &Settings) {
        self.present_time += settings.dt;
        self.present_step += 1;
        system::step_by_newton(system, settings.dt, settings.g);
    }

    pub fn simulate_to_end(&mut self, system: &mut SystemState, settings: &Settings) {
        for _ in 0..settings.n {
            self.iterate(system, settings);
        }
        if let Some(_) = settings.remainder_dt {
            self.iterate(system, settings);
        }
    }
}

pub struct Settings {
    g: f32,
    dt: f32,
    t: f32,
    n: u32,
    remainder_dt: Option<f32>,
}

impl Settings {
    pub fn new(g: f32, dt: f32, t: f32, n: u32, remainder_dt: Option<f32>) -> Settings {
        Settings {
            g,
            dt,
            t,
            n,
            remainder_dt,
        }
    }
}
