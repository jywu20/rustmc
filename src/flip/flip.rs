use crate::*;

pub trait Flip: EnergyMeasure + Model {
    fn sweep<F: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, callback: F);
    // fn propose(&self) -> usize;
}