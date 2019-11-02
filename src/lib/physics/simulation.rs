use super::model::{NewtonianModel};

pub struct Simulation {
    settings: SimulationSettings,
    physics_model: NewtonianModel,
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct SimulationSettings {
    dt: f32,
}

impl Simulation {
    pub fn of(physics_model: NewtonianModel, settings: SimulationSettings) -> Simulation {
        Simulation {
            settings,
            physics_model,
        }
    }
    pub fn do_steps(&mut self, steps: usize) -> &NewtonianModel {
        for _ in 0..steps {
            self.single_step();
        }
        &self.physics_model
    }

    fn single_step(&mut self) {
        self.physics_model.single_step_by(self.settings.dt);
    }
}

