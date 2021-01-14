pub trait EnergyMeasure {
    type ModelParameter;
    
    pub fn set_model_parameter(model_parameter: &ModelParameter);
    pub fn energy(&self) -> f64;
    pub fn energy_change(&self, flipped_site: usize) -> f64;
}