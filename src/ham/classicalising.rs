use crate::config::*;
use crate::lattice::IsingField;

pub struct ModelParameter {
    pub j: f64,
    pub beta: f64,
    pub b: f64
}

impl IsingField {
    pub fn energy_change(&self, flipped_site: usize, &ModelParameter {j, beta, b}: &ModelParameter) -> f64 {
        let mut delta_free_energy_int = 0.0;
        let mut delta_free_energy_b = 0.0;
        for i in 0 .. 4 {
            delta_free_energy_int += (self.configuration[flipped_site] * self.configuration[self.neighbor(flipped_site, i)]) as f64;
        }
        delta_free_energy_b += self.configuration[flipped_site] as f64;
        2.0 * beta * (delta_free_energy_int * j + delta_free_energy_b * b)
    }

    pub fn energy(&self, &ModelParameter {j, beta, b}: &ModelParameter) -> f64 {
        let mut delta_free_energy_int = 0.0;
        let mut delta_free_energy_b = 0.0;
        for site in 0 .. SITE_NUM {
            for i in 0 .. 2 {
                delta_free_energy_int += (self.configuration[site] * self.configuration[self.neighbor(site, i)]) as f64;
            }
            delta_free_energy_b += self.configuration[site] as f64;
        }
        - beta * (delta_free_energy_int * j + delta_free_energy_b * b)
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
                            let mut lattice = IsingField::new();
            
                            let free_energy_before = (&lattice).energy(&ModelParameter {j, beta, b});
                            let predicted_free_energy_change = (&lattice)
                                .energy_change(flipped_site, &ModelParameter {j, beta, b});
                            lattice.configuration[flipped_site] *= -1;
                            let free_energy_after = (&lattice).energy(&ModelParameter {j, beta, b});

                            assert!((free_energy_after - free_energy_before - predicted_free_energy_change).abs() < 0.01);
                        }
                    }
                }
            }
        }
    }
}