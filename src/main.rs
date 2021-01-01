mod ham;
mod lattice;
mod postop;
mod update;
mod config;
mod model;

use ham::*;
use lattice::*;
use update::*;
use postop::*;
use model::*;

fn main() {
    let mut lattice = IsingField::new();
    let sweep_times = 1000;
    let heat_up_times = 1000;
    let bin_size = 10;
    
    for i in 50 .. 500 {
        let temp = 0.01 * i as f64;
        let beta = 1.0 / temp;
        let b = 0.01;

        // A list of magnetization and correlation from all the bins
        let result: Vec<(f64, f64)> = lattice.run(&ModelParameter {
            j: 1.0, beta, b
        }, &SimulationParameters {
            sweep_times, heat_up_times, bin_size
        }, |lattice| {(
                // Measure the magnetization and the correlation between two points
                magnetization(&lattice.configuration), 
                correlation(&lattice.configuration,
                    (&lattice).index_list[0][1], (&lattice).index_list[5][5])
            )}, |data_series| {
            // What is done in binning: average the magnetization and the correlation in the bin.
            let acc = data_series.iter()
                .fold((0.0, 0), |acc, &x| {
                    (acc.0 + x.0, acc.1 + x.1)                    
                });
            (acc.0 / bin_size as f64, acc.1 as f64 / bin_size as f64)
        });
        
        let (mag_acc, corr_acc) = result.iter().fold((0.0, 0.0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1)
        });
        println!("{} {} {}", temp, mag_acc / result.len() as f64, corr_acc / result.len() as f64);
    }
}
