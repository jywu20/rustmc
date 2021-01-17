use std::ops::{Deref, DerefMut};

pub trait Sweep {
    fn new() -> Self;
    fn sweep<F: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, callback: F);
}

#[derive(Clone, Copy)]
pub struct SweepingParameters {
    pub sweep_times: usize,
    pub bin_size: usize,
    pub heat_up_times: usize
}

impl SweepingParameters {
    pub fn new() -> Self {
        Self {
            sweep_times: 0, bin_size: 1, heat_up_times: 0
        }
    }
}

pub struct SweepingModel<F: Sweep> {
    sweepable: F,
    simulation_parameter: SweepingParameters
}

impl<F> SweepingModel<F> where F: Sweep {
    pub fn set_sweeping_parameters(&mut self, sweeping_parameter: SweepingParameters) {
        self.simulation_parameter = sweeping_parameter;
    }

    pub fn run<T, S, C: Fn(&F) -> T, G: Fn(Vec<T>) -> S>(&mut self, diagnose: C, binning: G) -> Vec<S> {
        let mut result = Vec::new();
        let SweepingParameters { sweep_times, bin_size, heat_up_times } = self.simulation_parameter;

        self.sweepable.sweep(heat_up_times, |_|{});

        for _ in (0 .. sweep_times).step_by(bin_size) {
            let mut this_bin = Vec::new();
            self.sweepable.sweep(bin_size, |field| {
                this_bin.push(diagnose(field));
            });
            result.push(binning(this_bin));
        }
        
        result
    }
}

impl<F> SweepingModel<F> where F: Sweep {
    pub fn new() -> Self {
        Self {
            sweepable: F::new(), 
            simulation_parameter: SweepingParameters::new()
        }
    }
}

impl<F> Deref for SweepingModel<F> where F: Sweep {
    type Target = F;
    fn deref(&self) -> &Self::Target {
        &self.sweepable
    }
}

impl<F> DerefMut for SweepingModel<F> where F: Sweep {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sweepable
    }
}