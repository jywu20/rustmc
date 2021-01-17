pub trait EnergyMeasure {
    type ModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter);
    fn energy(&self) -> f64;
    fn energy_change(&self, flipped_site: usize) -> f64;
}
