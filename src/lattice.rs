use std::ops::{Index, IndexMut};

pub trait Lattice2D: Index<usize> + IndexMut<usize> {
    fn site_index_to_coordinate(&self, site_index: usize) -> (usize, usize);
    fn coordinate_to_site_index(&self, coordinate: (usize, usize)) -> usize;
}

pub trait Lattice3D: Index<usize> + IndexMut<usize> {
    fn site_index_to_coordinate(&self, site_index: usize) -> (usize, usize, usize);
    fn coordinate_to_site_index(&self, coordinate: (usize, usize, usize)) -> usize;
}

pub trait Lattice4D: Index<usize> + IndexMut<usize> {
    fn site_index_to_coordinate(&self, site_index: usize) -> (usize, usize, usize, usize);
    fn coordinate_to_site_index(&self, coordinate: (usize, usize, usize, usize)) -> usize;
}