pub trait MagneticModel {
    fn magnetization(&self) -> f64;
    fn correlation(&self, point1: usize, point2: usize) -> f64;
}
