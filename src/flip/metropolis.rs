use std::ops::{Deref, DerefMut};

use rand::Rng;
use crate::*;
use crate::config::*;

/// If a struct `F` implements `MetropolisFlip`, a Metropolis algorithm can be implemented by information provided
/// by it, and the algorithm is encapsulated in `Metropolis<F>`. `Metropolis<F>` does one sweep a time, while 
/// `SweepingModel<Metropolis<F>>` can automatically control heating up and binning.
/// 
/// Both `SweepingModel` and `Metropolis` implements `DerefMut` so methods invoked on any instance on them may be 
/// defined on `flipping_field`.
pub trait MetropolisFlip {
    fn new() -> Self;
    fn flip(&mut self, flipped_site: usize);
    fn accept_rate(&self, flipped_site: usize) -> f64;
}

pub struct Metropolis<F: MetropolisFlip> {
    flipping_field: F
}

impl<F> Deref for Metropolis<F> where F: MetropolisFlip {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.flipping_field
    }
}

impl<F> DerefMut for Metropolis<F> where F: MetropolisFlip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flipping_field
    }
}

impl<F> Sweep for Metropolis<F> where F: MetropolisFlip {
    fn new() -> Self {
        Self {flipping_field: F::new()}
    }

    fn sweep<C: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, mut callback: C) {
        let mut rng = rand::thread_rng();
        for _ in 0 .. sweep_times {
            for flipped_site in 0 .. SITE_NUM {
                if rng.gen::<f64>() < self.accept_rate(flipped_site) {
                    self.flip(flipped_site);
                }
            }
            callback(self);
        }
    }
}