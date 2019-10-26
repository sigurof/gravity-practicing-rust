use crate::gravity::physics::PhysicsModel;

pub struct Simulation {
    settings: SimulationSettings,
    physics_model: PhysicsModel,
}

impl Simulation {
    pub fn of(physics_model: PhysicsModel, settings: SimulationSettings) -> Simulation {
        Simulation {
            settings,
            physics_model,
        }
    }
    pub fn do_steps(&mut self, steps: usize) -> &PhysicsModel {
        for _ in 0..steps {
            self.single_step();
        }
        &self.physics_model
    }

    fn single_step(&mut self) {
        self.physics_model.single_step_by(self.settings.dt);
    }
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct SimulationSettings {
    dt: f32,
}
