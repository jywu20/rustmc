use std::ops::{Deref, DerefMut, Range};
use ising::ClassicalIsingField2D;
use rand::Rng;
use rand::thread_rng;
use crate::*;
use crate::observables::Energy;
use crate::config::*;
use crate::flip::*;
use super::IsingField2D;
use super::IndexRange;
use super::ClassicalIsingModelParameter;

pub struct ClassicalIsingModel2DFlipping {
    field: ClassicalIsingField2D
}

impl Deref for ClassicalIsingModel2DFlipping {
    type Target = ClassicalIsingField2D;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl DerefMut for ClassicalIsingModel2DFlipping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}

impl MetropolisUpdate for ClassicalIsingModel2DFlipping {
    type SweepingRange = Range<usize>;

    fn new() -> Self {
        Self {
            field: ClassicalIsingField2D::new()
        }
    }

    fn flip(&mut self, site: usize) {
        self.field[site] *= -1;
    }

    fn accept_rate(&self, flipped_site: usize) -> f64 {
        (- self.energy_change(flipped_site)).exp()
    }

    /// Just sweep every site in the order of their indexes. 
    fn sweep_range(&self) -> Self::SweepingRange {
        0 .. SITE_NUM
    } 
}

pub type ClassicalIsingModel2DMetropolis = SweepingModel<MetropolisAlgorithm<ClassicalIsingModel2DFlipping>>;

pub struct ClassicalIsingModel2DWolffUpdating {
    field: ClassicalIsingField2D,
}

impl Deref for ClassicalIsingModel2DWolffUpdating {
    type Target = ClassicalIsingField2D;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl DerefMut for ClassicalIsingModel2DWolffUpdating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}

impl WolffClusterUpdate for ClassicalIsingModel2DWolffUpdating {
    type ExpansionRange = IndexRange;

    fn new() -> Self {
        Self {
            field: ClassicalIsingField2D::new()
        }
    }

    fn flip(&mut self, site: usize) {
        self.field[site] *= -1;
    }

    fn start(&self) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0 .. SITE_NUM)
    }

    fn propose(&mut self, site: usize) -> IndexRange {
        let (i, j) = self.field.site_index_to_coordinate(site);
        IndexRange::new([
            self.field.index_list[(i + SIDE - 1) % SIDE][j],
            self.field.index_list[i][(j + 1) % SIDE],
            self.field.index_list[(i + 1) % SIDE][j],
            self.field.index_list[i][(j + SIDE - 1) % SIDE]
        ].iter())
    }

    fn accept_rate(&mut self, _: usize, _: usize) -> f64 {
        let ClassicalIsingModelParameter { j, beta, b } = self.field.model_parameter;
        // TODO: introduce the magnetic field
        1.0 - (-2.0 * beta * j)
    }
}

pub type ClassicalIsingModel2DWolff = SweepingModel<WolffClusterAlgorithm<ClassicalIsingModel2DWolffUpdating>>;

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
                            model.field[flipped_site] *= -1;
                            let free_energy_after = (&model).energy();

                            assert!((free_energy_after - free_energy_before - predicted_free_energy_change).abs() < 0.01);
                        }
                    }
                }
            }
        }
    }
}