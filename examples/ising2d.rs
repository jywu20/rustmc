use montecarlo::ising::*;
use montecarlo::*;

fn main() {
    let mut model = ClassicalIsingModel2D::new();
    let sweep_times = 1000;
    let heat_up_times = 1000;
    let bin_size = 10;

    for i in 50..500 {
        let temp = 0.01 * i as f64;
        let beta = 1.0 / temp;
        let b = 0.01;

        model.set_model_parameters(ClassicalIsingModelParameter { j: 1.0, beta, b });

        model.set_simulation_parameters(MetropolisParameters {
            sweep_times,
            heat_up_times,
            bin_size,
        });

        // A list of magnetization and correlation from all the bins
        let result: Vec<(f64, f64)> = model.run(
            |model| {
                (
                    // Measure the magnetization and the correlation between two points
                    model.lattice.magnetization(),
                    model.lattice.correlation(
                        (&model).lattice.index_list[0][1],
                        (&model).lattice.index_list[5][5],
                    )
                )
            },
            |data_series| {
                // What is done in binning: average the magnetization and the correlation in the bin.
                let acc = data_series
                    .iter()
                    .fold((0.0, 0.0), |acc, &x| (acc.0 + x.0, acc.1 + x.1));
                (acc.0 / bin_size as f64, acc.1 as f64 / bin_size as f64)
            },
        );

        let (mag_acc, corr_acc) = result
            .iter()
            .fold((0.0, 0.0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
            
        println!("{} {} {}", temp, mag_acc / result.len() as f64, corr_acc / result.len() as f64);
    }
}
