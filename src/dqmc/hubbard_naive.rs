use nalgebra::DMatrix;
use crate::{Sweep, config::SITE_NUM, ising::IsingField2D};

#[derive(Clone, Copy)]
pub struct HubbardModelParameter {
    pub t: f64,
    pub u: f64,
    pub beta: f64
}

impl HubbardModelParameter {
    pub fn new() -> Self {
        Self {
            t: 1.0, u: 0.0, beta: 1.0
        }
    }
}

/// This is an implementation of DQMC for Hubbard model solely depending on $\bold{B}$ matrices.
/// Needless to say, high numerical error will accumulate when the model runs, so the only practical usage
/// of this implementation is to check whether more optimized implementations are correct.
/// Standard Metropolis algorithm is used to update the model.
pub struct Hubbard2DDQMCNaiveFlipping {
    pub configuration: Vec<IsingField2D>,
    pub delta_tau: f64,
    pub time_slices: usize,
    pub parameters: HubbardModelParameter,
    alpha: f64
}

// Methods about initialization.
impl Hubbard2DDQMCNaiveFlipping {
    pub fn create_with_parameters(parameters: HubbardModelParameter, time_slices: usize) -> Self {
        let delta_tau = parameters.beta / (time_slices as f64);
        let mut configuration: Vec<IsingField2D> = Vec::with_capacity(time_slices);
        for _ in 0 .. time_slices {
            configuration.push(IsingField2D::new());
        }
        let HubbardModelParameter { u, t: _, beta: _ } = parameters;
        Self {
            delta_tau, configuration, parameters, time_slices,
            alpha: (delta_tau * u / 2.0).exp().acosh()
        }
    }

    /// Making the kinetic Hamiltonian under the coordinates representation.
    pub fn make_kinetic_matrix(&self) -> DMatrix<f64> {
        let mut kinetic_matrix = DMatrix::repeat(SITE_NUM, SITE_NUM, 0.0);
        for site in 0 .. SITE_NUM {
            // TODO: the following lines are kind of ugly ...
            let ref first_field = self.configuration[0];
            for &nearest in first_field.nearest_neighbor(site).iter() {
                kinetic_matrix[(site, nearest)] = -1.0;
                kinetic_matrix[(nearest, site)] = -1.0;
            }
        }
        kinetic_matrix
    }
}

// Methods used to compute B matrices and accept rates.
impl Hubbard2DDQMCNaiveFlipping {
    pub fn exp_kin_spin_up(&self) {
        
    }

    /// Exponents of B matrix with up spins.
    pub fn exp_b_mat_spin_up(&self, n_tau: usize) {
        let alpha = self.alpha;

    }

    pub fn accept_rate(&self, site: usize) {

    }
}

impl Sweep for Hubbard2DDQMCNaiveFlipping {
    fn new() -> Self {
        todo!()
    }

    fn sweep<F: FnMut(&Self) -> ()>(&mut self, sweep_times: usize, mut callback: F) {
        for _ in 0 .. sweep_times {

            // The callback function is invoked after each sweep is finished.
            callback(self);
        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use nalgebra::DMatrix;
    use super::*;
    use factorial::*;

    #[test]
    fn matrix() {
        let m1 = DMatrix::from_row_slice(2, 2, &[-1.0, 1.0, 1.0, -1.0]);
        let m2 = DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, 0.0]);
        println!("{}", &m1 * &m2);
    }

    #[test]
    fn kinetic_matrix() {
        let model = Hubbard2DDQMCNaiveFlipping::create_with_parameters(HubbardModelParameter::new(), 10);
        println!("{}", model.make_kinetic_matrix());
    }

    fn exp_mat_naive(m: DMatrix<f64>, iteration_times: usize) -> DMatrix<f64> {
        let (l, l2) = m.shape();
        if l != l2 {
            panic!("The exp function can only be applied to a square matrix.");
        }
        
        let mut result = DMatrix::repeat(l, l, 0.0);
        result.fill_with_identity();
        
        for n in 1 .. iteration_times {
            let mut this_term = m.clone();
            for _ in 1 .. n {
                this_term *= &m;
            }
            result += this_term / (n.factorial() as f64);
        }

        result
    }

    #[test]
    fn test_exp_mat() {
        let mut m = DMatrix::repeat(3, 3, 0.0);
        m.fill_with_identity();
        println!("{}", exp_mat_naive(m, 10));
        println!("{}", std::f64::consts::E)
    }
}