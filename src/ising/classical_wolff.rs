use std::ops::{Deref, DerefMut};
use ising::ClassicalIsingField2D;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::*;
use crate::config::*;
use crate::flip::*;
use super::IndexRange;
use super::ClassicalIsingModelParameter;

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
        let mut rng = thread_rng();
        let (i, j) = self.field.site_index_to_coordinate(site);
        let mut indexes = [
            self.field.index_list[(i + SIDE - 1) % SIDE][j],
            self.field.index_list[i][(j + 1) % SIDE],
            self.field.index_list[(i + 1) % SIDE][j],
            self.field.index_list[i][(j + SIDE - 1) % SIDE]
        ];
        indexes.shuffle(&mut rng);
        IndexRange::new(indexes.iter())
    }

    fn accept_prob(&self, point_1: usize, point_2: usize) -> f64 {
        let ClassicalIsingModelParameter { j, beta, b } = self.field.model_parameter;
        // TODO: introduce the magnetic field
        if self[point_1] == self[point_2] {
            1.0 - (-2.0 * beta * j).exp()
        } else {
            0.0
        }
    }
}

pub type ClassicalIsingModel2DWolff = SweepingModel<WolffClusterAlgorithm<ClassicalIsingModel2DWolffUpdating>>;