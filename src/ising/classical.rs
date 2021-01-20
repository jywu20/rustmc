use std::ops::{Deref, DerefMut};
use crate::observables::Energy;
use crate::config::SITE_NUM;
use super::IsingField2D;

#[derive(Copy, Clone)]
pub struct ClassicalIsingModelParameter {
    pub j: f64,
    pub beta: f64,
    pub b: f64
}

impl ClassicalIsingModelParameter {
    pub fn new() -> Self {
        ClassicalIsingModelParameter {
            j: 0.0, beta: 0.0, b: 0.0
        }
    }
}

pub struct ClassicalIsingField2D {
    pub lattice: IsingField2D,
    pub model_parameter: ClassicalIsingModelParameter
}

impl ClassicalIsingField2D {
    pub fn new() -> Self {
        Self {
            lattice: IsingField2D::new(), model_parameter: ClassicalIsingModelParameter::new()
        }
    }
}

impl Deref for ClassicalIsingField2D {
    type Target = IsingField2D;

    fn deref(&self) -> &Self::Target {
        &self.lattice
    }
}

impl DerefMut for ClassicalIsingField2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lattice
    }
}

impl Energy for ClassicalIsingField2D {
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
                (self[flipped_site] * self[self.neighbor(flipped_site, i)]) as f64;
        }
        
        delta_free_energy_b += self[flipped_site] as f64;

        2.0 * beta * (delta_free_energy_int * j + delta_free_energy_b * b)
    }

    fn energy(&self) -> f64 {
        let ClassicalIsingModelParameter {j, beta, b} = self.model_parameter;
        let mut delta_free_energy_int = 0.0;
        let mut delta_free_energy_b = 0.0;
        for site in 0 .. SITE_NUM {
            for i in 0 .. 2 {
                delta_free_energy_int += (self[site] * self[self.neighbor(site, i)]) as f64;
            }
            delta_free_energy_b += self[site] as f64;
        }

        - beta * (delta_free_energy_int * j + delta_free_energy_b * b)
    }
}
