pub trait Energy {
    type ModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter);
    fn energy(&self) -> f64;
    fn energy_change(&self, flipped_site: usize) -> f64;
}

pub trait Magnetic {
    fn magnetization(&self) -> f64;
    fn correlation(&self, point1: usize, point2: usize) -> f64;
}