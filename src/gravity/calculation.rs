use crate::gravity::physical::system;
use nalgebra::Vector3 as v3;
use system::SystemState;

pub struct Recorder {
    /*     calc: Calculator,
    settings: Settings,
    systemState: SystemState, */
    positions: Vec<Vec<v3<f32>>>,
    system_state: SystemState,
    settings: Settings,
    n_frames: usize,
    steps_per_frame: usize,
}

impl Recorder {
    pub fn of(system_state: SystemState, settings: Settings, steps_per_frame: usize) -> Recorder {
        let n_frames = f32::floor((settings.n as f32) / (steps_per_frame as f32)) as usize;
        Recorder {
            positions: vec![vec![v3::new(0.0, 0.0, 0.0); system_state.getBodies().len()]; n_frames],
            system_state,
            settings,
            n_frames,
            steps_per_frame,
        }
    }

    pub fn get_positions(&self) -> &Vec<Vec<v3<f32>>> {
        return &self.positions;
    }

    pub fn get_n_frames(&self) -> usize {
        return self.n_frames;
    }

    pub fn record(&mut self) {
        let mut calc = Calculator::new();
        for frame_number in 0..self.n_frames {
            self.record_frame(frame_number);
            for _ in 0..self.steps_per_frame {
                calc.iterate(&mut self.system_state, &self.settings);
            }
        }
        if let Some(_) = self.settings.remainder_dt {
            calc.iterate(&mut self.system_state, &self.settings);
        }
    }

    fn record_frame(&mut self, frame_number: usize) {
        let bodies = self.system_state.getBodies();
        for (i, body) in self.system_state.getBodies().iter().enumerate() {
            self.positions[frame_number][i] = body.get_position();
        }
    }
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

    pub fn iterate(&mut self, system: &mut SystemState, settings: &Settings) {
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
