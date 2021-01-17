pub trait EnergyMeasure {
    type ModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter);
    fn energy(&self) -> f64;
    fn energy_change(&self, flipped_site: usize) -> f64;
}

pub trait Weight {
    type ModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter);
    fn weight(&self) -> f64;
    fn weight_change(&self, flipped_site: usize) -> f64;
}

pub trait Flip: EnergyMeasure {
    fn new() -> Self;
    fn flip(&mut self, site: usize);
    fn sweep<F: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, callback: F);
}

