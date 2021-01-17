use crate::ising::IsingField2D;

pub struct HubbardDQMC {
    /// A `Vec` is used here because the exact length on the imaginary time dimension depends on $\Delta \tau$
    /// and hence is not known during the compile time.
    pub configuration: Vec<IsingField2D>
}
