pub trait EnergyMeasure {
    type ModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter);
    fn energy(&self) -> f64;
    fn energy_change(&self, flipped_site: usize) -> f64;
}

pub trait Model {
    type SimulationParameters;

    fn set_simulation_parameters(&mut self, simulation_parameter: Self::SimulationParameters);
    fn run<T, S, F: Fn(&Self) -> T, G: Fn(Vec<T>) -> S>(
        &mut self, diagnose: F, binning: G,
    ) -> Vec<S>;
}