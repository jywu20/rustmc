/// Updating method based on flipping sites on a lattice and using the energy to decide whether the change is accepted.
mod metropolis;
mod cluster;

pub use metropolis::*;
pub use cluster::*;