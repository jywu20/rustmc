use std::ops::{Index, IndexMut};

use crate::{config::SITE_NUM, worldline::*};

#[derive(PartialEq, Eq)]
pub enum XXY1DWorldlineComponents {
    Straight, Left, Right
}

pub struct XXY1DWorldline {
    worldlines: Vec<Vec<XXY1DWorldlineComponents>>,
    starting_points: Vec<usize>
}

impl XXY1DWorldline {
    pub fn position_now(&self, worldline_index: usize, time_steps: usize) -> usize {
        let mut position = self.starting_points[worldline_index] as i32;
        let ref worldline = self.worldlines[worldline_index];
        for i in 0 .. time_steps {
            match worldline[i] {
                XXY1DWorldlineComponents::Left => position -= 1,
                XXY1DWorldlineComponents::Right => position += 1,
                XXY1DWorldlineComponents::Straight => ()
            };
        }
        // TODO: change into generic const
        position.rem_euclid(SITE_NUM as i32) as usize
    }
}

impl Index<usize> for XXY1DWorldline {
    type Output = Vec<XXY1DWorldlineComponents>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.worldlines[index]
    }
}

impl IndexMut<usize> for XXY1DWorldline {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.worldlines[index]
    }
}

impl Worldline1D for XXY1DWorldline {
    fn new() -> Self {
        
        todo!()
    }

    fn is_well_formed() -> bool {
        todo!()
    }

    fn try_add(&mut self, start_point: usize) -> bool {
        if self.starting_points.contains(&start_point) {
            return false;
        }
        todo!()
    }

    fn delete(&mut self, index: usize) {
        self.worldlines.remove(index);
        self.starting_points.remove(index);
    }
}