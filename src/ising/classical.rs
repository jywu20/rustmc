use rand::Rng;
use crate::flip::EnergyMeasure;
use crate::config::*;
use crate::flip::*;
use super::IsingField2D;

#[derive(Copy, Clone)]
pub struct ClassicalIsingModelParameter {
    pub j: f64,
    pub beta: f64,
    pub b: f64
}

pub struct ClassicalIsingModel2DFlipping {
    pub lattice: IsingField2D,
    pub model_parameter: ClassicalIsingModelParameter,
    pub simulation_parameter: MetropolisParameters
}

impl EnergyMeasure for ClassicalIsingModel2DFlipping {
    type ModelParameter = ClassicalIsingModelParameter;

    fn set_model_parameters(&mut self, model_parameter: Self::ModelParameter) {
        self.model_parameter = model_parameter;
    }

    fn energy_change(&self, flipped_site: usize) -> f64 {
        let ClassicalIsingModelParameter {j, beta, b} = self.model_parameter;

        let mut delta_free_energy_int = 0.0;
        let mut delta_free_energy_b = 0.0;
        
        for i in 0 .. 4 {
            delta_free_energy_int += 
                (self.lattice[flipped_site] * self.lattice[self.lattice.neighbor(flipped_site, i)]) as f64;
        }
        
        delta_free_energy_b += self.lattice[flipped_site] as f64;

        2.0 * beta * (delta_free_energy_int * j + delta_free_energy_b * b)
    }

    fn energy(&self) -> f64 {
        let ClassicalIsingModelParameter {j, beta, b} = self.model_parameter;
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

impl Flip for ClassicalIsingModel2DFlipping {
    fn new() -> Self {
        Self {
            lattice: IsingField2D::new(),
            model_parameter: ClassicalIsingModelParameter {
                j: 0.0, beta: 0.0, b: 0.0
            },
            simulation_parameter: MetropolisParameters {
                sweep_times: 0, bin_size: 1, heat_up_times: 0
            }
        }
    }

    fn flip(&mut self, site: usize) {
        self.lattice[site] *= -1;
    }

    fn sweep<F: FnMut(&ClassicalIsingModel2DFlipping) -> ()>(&mut self, sweep_times: usize, mut callback: F) {
        let mut rng = rand::thread_rng();
        for _ in 0 .. sweep_times {
            for flipped_site in 0 .. SITE_NUM {
                if rng.gen::<f64>() < (- self.energy_change(flipped_site)).exp() {
                    self.flip(flipped_site);
                }
            }
            callback(self);
        }
    }
}

pub type ClassicalIsingModel2DMetropolis = MetropolisUpdater<ClassicalIsingModel2DFlipping>;

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
                            let mut model = ClassicalIsingModel2DFlipping::new();
                            model.set_model_parameters(ClassicalIsingModelParameter {j, beta, b});

                            let free_energy_before = (&model).energy();
                            let predicted_free_energy_change = (&model).energy_change(flipped_site);
                            model.lattice[flipped_site] *= -1;
                            let free_energy_after = (&model).energy();

                            assert!((free_energy_after - free_energy_before - predicted_free_energy_change).abs() < 0.01);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_update() {
        let mut model = ClassicalIsingModel2DFlipping::new();
        let sweep_times = 10;
        model.set_model_parameters(ClassicalIsingModelParameter {
            j: 1.0, beta: 0.1, b: 0.0
        });
        model.sweep(sweep_times, |model| {
            println!("{}", model.lattice.to_string());
        });
    }
}