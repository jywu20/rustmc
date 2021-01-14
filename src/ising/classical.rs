use rand::Rng;
use crate::config::*;
use crate::postop;
use super::IsingField;

pub struct ModelParameter {
    pub j: f64,
    pub beta: f64,
    pub b: f64
}

pub struct ClassicalIsingModel {
    pub lattice: IsingField
}

impl ClassicalIsingModel {
    pub fn new() -> Self {
        Self {
            lattice: IsingField::new()
        }
    }
}

impl ClassicalIsingModel {
    pub fn energy_change(&self, flipped_site: usize, &ModelParameter {j, beta, b}: &ModelParameter) -> f64 {
        let mut delta_free_energy_int = 0.0;
        let mut delta_free_energy_b = 0.0;
        for i in 0 .. 4 {
            delta_free_energy_int += (self.lattice[flipped_site] * self.lattice[self.lattice.neighbor(flipped_site, i)]) as f64;
        }
        delta_free_energy_b += self.lattice[flipped_site] as f64;
        2.0 * beta * (delta_free_energy_int * j + delta_free_energy_b * b)
    }

    pub fn energy(&self, &ModelParameter {j, beta, b}: &ModelParameter) -> f64 {
        let mut delta_free_energy_int = 0.0;
        let mut delta_free_energy_b = 0.0;
        for site in 0 .. SITE_NUM {
            for i in 0 .. 2 {
                delta_free_energy_int += (self.lattice[site] * self.lattice[self.lattice.neighbor(site, i)]) as f64;
            }
            delta_free_energy_b += self.lattice[site] as f64;
        }
        - beta * (delta_free_energy_int * j + delta_free_energy_b * b)
    }
}

pub struct SimulationParameters {
    pub sweep_times: usize,
    pub bin_size: usize,
    pub heat_up_times: usize
}

impl ClassicalIsingModel {
    pub fn sweep_ising<F: FnMut(&ClassicalIsingModel) -> ()>(&mut self, sweep_times: usize, model_param: &ModelParameter, mut callback: F) {
        let mut rng = rand::thread_rng();
        for _ in 0 .. sweep_times {
            for flipped_site in 0 .. SITE_NUM {
                if rng.gen::<f64>() < (- self.energy_change(flipped_site, model_param)).exp() {
                    self.lattice[flipped_site] *= -1;
                }
            }
            callback(self);
        }
    }

    pub fn run<T, S, F: Fn(&ClassicalIsingModel) -> T, G: Fn(Vec<T>) -> S>(&mut self, model_param: &ModelParameter, sim_param: &SimulationParameters, diagnose: F, binning: G) -> Vec<S> {
        let mut result = Vec::new();
        
        self.sweep_ising(sim_param.heat_up_times, model_param, |_|{});

        for _ in (0 .. sim_param.sweep_times).step_by(sim_param.bin_size) {
            let mut this_bin = Vec::new();
            self.sweep_ising(sim_param.bin_size, model_param, |lattice| {
                this_bin.push(diagnose(lattice));
            });
            result.push(binning(this_bin));
        }
        
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_free_energy_change() {
        println!("Testing free_energy change.");

        for _ in 0 .. 20 {

            for flipped_site in 0 .. 4 {
                for &b in [1.0, -10.0, 0.0].iter() {
                    for &j in [-1.0, 0.0, 8.0].iter() {
                        for &beta in [0.9, 0.8, 0.1].iter() {
                            let mut model = ClassicalIsingModel::new();
            
                            let free_energy_before = (&model).energy(&ModelParameter {j, beta, b});
                            let predicted_free_energy_change = (&model)
                                .energy_change(flipped_site, &ModelParameter {j, beta, b});
                            model.lattice[flipped_site] *= -1;
                            let free_energy_after = (&model).energy(&ModelParameter {j, beta, b});

                            assert!((free_energy_after - free_energy_before - predicted_free_energy_change).abs() < 0.01);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_update() {
        let mut model = ClassicalIsingModel::new();
        let sweep_times = 10;
        model.sweep_ising(sweep_times, &ModelParameter {
            j: 1.0, beta: 0.1, b: 0.0
        }, |model| {
            println!("{}", model.lattice.to_string());
        });
    }
}