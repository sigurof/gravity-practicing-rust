use super::model::{NewtonianModel, PhysicsModel};

pub struct Simulation<'a> {
    settings: SimulationSettings,
    physics_model: Box<dyn PhysicsModel + 'a>,
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct SimulationSettings {
    dt: f32,
}

impl<'a> Simulation<'a> {
    pub fn of(
        physics_model: impl PhysicsModel + 'a,
        settings: SimulationSettings,
    ) -> Simulation<'a> {
        Simulation {
            settings,
            physics_model: Box::from(physics_model),
        }
    }
    pub fn do_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.single_step();
        }
    }

    pub fn get_physics_model(&self) -> &Box<dyn PhysicsModel + 'a> {
        &self.physics_model
    }

    fn single_step(&mut self) {
        self.physics_model.single_step_by(self.settings.dt);
    }
}
