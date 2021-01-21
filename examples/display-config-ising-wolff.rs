use colored::*;
use montecarlo::Sweep;
use montecarlo::config::*;
use montecarlo::observables::{Energy, Magnetic};
use montecarlo::ising::{
    ClassicalIsingModel2DWolff, 
    ClassicalIsingModelParameter,
    ClassicalIsingField2D
};

fn show_colored_field(field: &ClassicalIsingField2D) {
    for i in 0 .. SIDE {
        for j in 0 .. SIDE {
            if field.configuration[field.index_list[i][j]] == -1 {
                print!("{}", "-1 ".on_red());
            } else {
                print!("{}", " 1 ".on_blue());
            }
        }
        println!();
    }
    println!();
}    

fn main() {
    let mut model = ClassicalIsingModel2DWolff::new();

    for i in 0 .. SITE_NUM {
        model[i] = 1;
    }
    show_colored_field(&model);

    model.set_model_parameters(ClassicalIsingModelParameter {
        j: 1.0, beta: 5.0, b: 0.0
    });

    for i in 0 .. 1000 {
        model.sweep(1, |_| {});
        if i % 100 == 0 {
            println!("Energy: {}", model.energy());
            println!("Magnetization: {}", model.magnetization());
            show_colored_field(&model);
            println!();
        }
    }
}