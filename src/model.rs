pub trait Model {
    type SimulationParameters;
    type FieldConfiguration;

    fn set_simulation_parameters(&mut self, simulation_parameter: Self::SimulationParameters);
    fn run<T, S, F: Fn(&Self::FieldConfiguration) -> T, G: Fn(Vec<T>) -> S>(
        &mut self, diagnose: F, binning: G,
    ) -> Vec<S>;
}