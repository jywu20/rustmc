use super::lattice::*;
use super::ham::*;
use super::config::*;
use rand::Rng;

// Things like SIDE and SITE_NUM should have be included in this struct but since generic const isn't stable we have to wait.
pub struct SimulationParameters {
    pub sweep_times: usize,
    pub bin_size: usize,
    pub heat_up_times: usize
}

impl Lattice {
    pub fn sweep_times<F: FnMut(&Lattice) -> ()>(&mut self, sweep_times: usize, model_param: &ModelParameter, mut callback: F) {
        let mut rng = rand::thread_rng();
        for _ in 0 .. sweep_times {
            for flipped_site in 0 .. SITE_NUM {
                if rng.gen::<f64>() < (- self.free_energy_change(flipped_site, model_param)).exp() {
                    self.configuration[flipped_site] *= -1;
                }
            }
            callback(self);
        }
    }

    pub fn run<T, S, F: Fn(&Lattice) -> T, G: Fn(Vec<T>) -> S>(&mut self, model_param: &ModelParameter, sim_param: &SimulationParameters, diagnose: F, binning: G) -> Vec<S> {
        let mut result = Vec::new();
        
        self.sweep_times(sim_param.heat_up_times, model_param, |_|{});

        for _ in (0 .. sim_param.sweep_times).step_by(sim_param.bin_size) {
            let mut this_bin = Vec::new();
            self.sweep_times(sim_param.bin_size, model_param, |lattice| {
                this_bin.push(diagnose(lattice));
            });
            result.push(binning(this_bin));
        }
        
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update() {
        let mut lattice = Lattice::new();
        let sweep_times = 10;
        lattice.sweep_times(sweep_times, &ModelParameter {
            j: 1.0, beta: 0.1, b: 0.0
        }, |lattice| {
            println!("{}", lattice.to_string());
        });
    }

    // #[test]
    // fn test_run() {
    //     let mut lattice = Lattice::new();
    //     lattice.run(&ModelParameter {
    //         j: 1.0, beta: 0.1
    //     }, &SimulationParameters {
    //         sweep_times: 1000, bin_size: 10
    //     }, |lattice| {
    //         
    //     }, |data_series| {
    //         
    //     });
    // }
}