use std::ops::{Deref, DerefMut, Range};
use ising::ClassicalIsingField2D;
use crate::*;
use crate::observables::Energy;
use crate::config::*;
use crate::flip::*;

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

    fn accept_prob(&self, flipped_site: usize) -> f64 {
        (- self.energy_change(flipped_site)).exp()
    }

    /// Just sweep every site in the order of their indexes. 
    fn sweep_range(&self) -> Self::SweepingRange {
        0 .. SITE_NUM
    } 
}

pub type ClassicalIsingModel2DMetropolis = SweepingModel<MetropolisAlgorithm<ClassicalIsingModel2DFlipping>>;
