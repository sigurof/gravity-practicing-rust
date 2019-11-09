use super::model::{old_newton::NewtonianModel, PhysicsModel};

pub struct Simulation<'a, T> {
    settings: SimulationSettings,
    physics_model: Box<dyn PhysicsModel<T> + 'a>,
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct SimulationSettings {
    dt: f32,
}

impl<'a, T> Simulation<'a, T> {
    pub fn of(
        physics_model: impl PhysicsModel<T> + 'a,
        settings: SimulationSettings,
    ) -> Simulation<'a, T> {
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

    pub fn expose_state(&self) -> &T {
        self.physics_model.expose_state()
    }

    pub fn get_physics_model(&self) -> &Box<dyn PhysicsModel<T> + 'a> {
        &self.physics_model
    }

    fn single_step(&mut self) {
        self.physics_model.single_step_by(self.settings.dt);
    }
}
