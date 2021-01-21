use nalgebra::Matrix;
use crate::ising::IsingField2D;

pub struct HubbardDQMC {
    /// A `Vec` is used here because the exact length on the imaginary time dimension depends on $\Delta \tau$
    /// and hence is not known during the compile time.
    pub configuration: Vec<IsingField2D>
}

#[cfg(test)]
mod test {
    use std::ops::Add;
    use nalgebra::DMatrix;
    use super::*;

    #[test]
    fn matrix() {
        let m1 = DMatrix::from_row_slice(2, 2, &[-1.0, 1.0, 1.0, -1.0]);
        let m2 = DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, 0.0]);
        println!("{}", &m1 * &m2);
    }

}