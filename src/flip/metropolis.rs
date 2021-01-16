use crate::Model;

use super::{EnergyMeasure, Flip};

#[derive(Clone, Copy)]
pub struct MetropolisParameters {
    pub sweep_times: usize,
    pub bin_size: usize,
    pub heat_up_times: usize
}

impl MetropolisParameters {
    pub fn new() -> Self {
        Self {
            sweep_times: 0, bin_size: 1, heat_up_times: 0
        }
    }
}

pub struct MetropolisUpdater<F: Flip> {
    sweepable_field: F,
    simulation_parameter: MetropolisParameters
}

impl<F> MetropolisUpdater<F> where F: Flip {
    pub fn new() -> Self {
        Self {
            sweepable_field: F::new(), 
            simulation_parameter: MetropolisParameters::new()
        }
    }
}

impl<F> EnergyMeasure for MetropolisUpdater<F> where F: Flip {
    type ModelParameter = F::ModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter) {
        self.sweepable_field.set_model_parameters(model_parameter);
    }

    fn energy(&self) -> f64 {
        self.sweepable_field.energy()
    }

    fn energy_change(&self, flipped_site: usize) -> f64 {
        self.sweepable_field.energy_change(flipped_site)
    }
}

impl<F> Model for MetropolisUpdater<F> where F: Flip {
    type SimulationParameters = MetropolisParameters;
    type FieldConfiguration = F;

    fn set_simulation_parameters(&mut self, simulation_parameter: Self::SimulationParameters) {
        self.simulation_parameter = simulation_parameter;
    }

    fn run<T, S, C: Fn(&F) -> T, G: Fn(Vec<T>) -> S>(&mut self, diagnose: C, binning: G) -> Vec<S> {
        let mut result = Vec::new();
        let MetropolisParameters { sweep_times, bin_size, heat_up_times } = self.simulation_parameter;

        self.sweepable_field.sweep(heat_up_times, |_|{});

        for _ in (0 .. sweep_times).step_by(bin_size) {
            let mut this_bin = Vec::new();
            self.sweepable_field.sweep(bin_size, |field| {
                this_bin.push(diagnose(field));
            });
            result.push(binning(this_bin));
        }
        
        result
    }
}