use montecarlo::*;
use montecarlo::observables::*;
use montecarlo::ising::*;

fn main() {
    let sweep_times = 1000;
    let heat_up_times = 1000;
    let bin_size = 10;

    for i in 50..500 {
        let temp = 0.01 * i as f64;
        let beta = 1.0 / temp;
        let b = 0.0;

        let mut model = ClassicalIsingModel2DWolff::new();
        // println!("{}", model.to_string());

        model.set_model_parameters(ClassicalIsingModelParameter { j: 1.0, beta, b });

        model.set_sweeping_parameters(SweepingParameters {
            sweep_times,
            heat_up_times,
            bin_size,
        });

        // A list of magnetization and correlation from all the bins
        let result: Vec<(f64, f64)> = model.run(
            |model| {
                (
                    // Measure the magnetization and the correlation between two points

                    // With cluster updating, when a ferromagnetic order is formed, the whole ising field is a 
                    // big cluster, so the magnetization flips rapidly between 1 and -1.
                    // That is why `.abs()` is necessary here.
                    model.magnetization().abs(),
                    model.correlation(
                        (&model).index_list[0][1],
                        (&model).index_list[5][5],
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

        // println!("{}", model.to_string());

        let (mag_acc, corr_acc) = result
            .iter()
            .fold((0.0, 0.0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        
        println!("{} {} {}", temp, mag_acc / result.len() as f64, corr_acc / result.len() as f64);
    }
}
