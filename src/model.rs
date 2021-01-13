use super::lattice::IsingField;
use super::ham::ModelParameter;
use super::update::SimulationParameters;

pub trait Sweep {
    fn run<T, S, F: Fn(&IsingField) -> T, G: Fn(Vec<T>) -> S>(&mut self, model_param: &ModelParameter, sim_param: &SimulationParameters, diagnose: F, binning: G) -> Vec<S>;
}

#[cfg(test)]
mod test {
    
}