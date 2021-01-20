use std::ops::{Deref, DerefMut};
use rand::Rng;
use crate::*;

/// If a struct `F` implements `MetropolisFlip`, a Metropolis algorithm can be implemented by information provided
/// by it, and the algorithm is encapsulated in `Metropolis<F>`. `Metropolis<F>` does one sweep a time, while 
/// `SweepingModel<Metropolis<F>>` can automatically control heating up and binning.
/// 
/// Both `SweepingModel` and `Metropolis` implements `DerefMut` so methods invoked on any instance on them may be 
/// defined on `flipping_field`. Quite often, `flipping_field` is just a wrapper of a field, providing methods for
/// the `MetropolisUpdate` trait, so itself implements `Deref` and `DerefMut`.
pub trait MetropolisUpdate where <Self::SweepingRange as Iterator>::Item : Copy {
    type SweepingRange: Iterator;
    
    fn new() -> Self;
    fn flip(&mut self, flipped_site: <Self::SweepingRange as Iterator>::Item);
    fn accept_rate(&self, flipped_site: <Self::SweepingRange as Iterator>::Item) -> f64;
    fn sweep_range(&self) -> Self::SweepingRange;
}

pub struct MetropolisAlgorithm<F: MetropolisUpdate> where <F::SweepingRange as Iterator>::Item : Copy {
    flipping_field: F
}

impl<F> Deref for MetropolisAlgorithm<F> where F: MetropolisUpdate, <F::SweepingRange as Iterator>::Item : Copy {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.flipping_field
    }
}

impl<F> DerefMut for MetropolisAlgorithm<F> where F: MetropolisUpdate, <F::SweepingRange as Iterator>::Item : Copy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flipping_field
    }
}

impl<F> Sweep for MetropolisAlgorithm<F> where F: MetropolisUpdate, <F::SweepingRange as Iterator>::Item : Copy {
    fn new() -> Self {
        Self {flipping_field: F::new()}
    }

    fn sweep<C: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, mut callback: C) {
        let mut rng = rand::thread_rng();
        for _ in 0 .. sweep_times {
            for flipped_site in self.flipping_field.sweep_range() {
                if rng.gen::<f64>() < self.accept_rate(flipped_site) {
                    self.flip(flipped_site);
                }
            }
            callback(self);
        }
    }
}
