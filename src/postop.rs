/// Post-process.

use super::config::*;

pub fn magnetization(configuration: &[i32;SITE_NUM]) -> f64 {
    (configuration.iter().sum::<i32>() as f64) / (SITE_NUM as f64)
}

pub fn correlation(configuration: &[i32;SITE_NUM], point1: usize, point2: usize) -> i32 {
    configuration[point1] * configuration[point2]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lattice::*;

    #[test]
    fn test_magnetization() {
        println!("Test magnetization.");
        let mut lattice = Lattice::new();

        for i in 0 .. SITE_NUM {
            lattice.configuration[i] = 1;
        }
        println!("{}", magnetization(&lattice.configuration));

        for i in 0 .. SITE_NUM {
            lattice.configuration[i] = -1;
        }
        println!("{}", magnetization(&lattice.configuration));

        for i in 0 .. SITE_NUM {
            lattice.configuration[i] = (-1 as i32).pow(i as u32);
        }

        println!("{}", magnetization(&lattice.configuration));

        for _ in 0 .. 100 {
            let lattice = Lattice::new();
            println!("{}", magnetization(&lattice.configuration));
        }
    }
}