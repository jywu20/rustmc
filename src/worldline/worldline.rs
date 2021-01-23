use std::ops::{Index, IndexMut};

// TODO: when const generic is stable, put the size of the imaginary dimension into the type declarations.

/// World line for 1+1 dimensional systems.
pub trait Worldline1D: Index<usize> + IndexMut<usize> {
    fn new() -> Self;
    /// Check whether the whole configuration is well-formed.
    /// For example, whether the periodic boundary condition in the imaginary dimension is satisfied.
    fn is_well_formed() -> bool;
    /// Try to randomly insert a world line from a start point.
    /// The return value is whether a world line is truly inserted, since there is possibility that no 
    /// world line can be further created.
    fn try_add(&mut self, start_point: usize) -> bool;
    /// Delete a world line.
    fn delete(&mut self, index: usize);
}
