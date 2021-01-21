use std::ops::{Deref, DerefMut};
use rand::Rng;
use crate::*;

pub trait WolffClusterUpdate where <Self::ExpansionRange as Iterator>::Item : Copy + Eq {
    type ExpansionRange: Iterator;

    fn new() -> Self;
    fn start(&self) -> <Self::ExpansionRange as Iterator>::Item;
    fn propose(&mut self, site: <Self::ExpansionRange as Iterator>::Item) -> Self::ExpansionRange;
    fn accept_prob(&self, start: <Self::ExpansionRange as Iterator>::Item, end: <Self::ExpansionRange as Iterator>::Item) -> f64;
    fn flip(&mut self, site: <Self::ExpansionRange as Iterator>::Item);
}

pub struct WolffClusterAlgorithm<F> where F: WolffClusterUpdate, <F::ExpansionRange as Iterator>::Item : Copy + Eq {
    field: F
}

impl<F> Deref for WolffClusterAlgorithm<F> where F: WolffClusterUpdate, <F::ExpansionRange as Iterator>::Item : Copy + Eq {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<F> DerefMut for WolffClusterAlgorithm<F> where F: WolffClusterUpdate, <F::ExpansionRange as Iterator>::Item : Copy + Eq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}

impl<F> Sweep for WolffClusterAlgorithm<F> where F: WolffClusterUpdate, <F::ExpansionRange as Iterator>::Item : Copy + Eq {
    fn new() -> Self {
        Self {
            field: F::new()
        }
    }

    fn sweep<C: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, mut callback: C) {
        for _ in 0 .. sweep_times {
            let mut rng = rand::thread_rng();
            
            let mut cluster: Vec<<F::ExpansionRange as Iterator>::Item> = Vec::new(); 
            let mut to_be_considered_centers: Vec<<F::ExpansionRange as Iterator>::Item> = Vec::new();
            
            let start = self.start();
            to_be_considered_centers.push(start);
            cluster.push(start);

            // Form a cluster.
            loop {
                if let Some(center) = to_be_considered_centers.pop() {
                    for site in self.propose(center) {
                        if ! cluster.contains(&site) {
                            if rng.gen::<f64>() < self.accept_prob(center, site) {
                                cluster.push(site);
                                to_be_considered_centers.push(site);
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            // Flip the cluster.
            for site in cluster {
                self.flip(site);
            }
            callback(self);
        }
    }
}
