/// Returns a random real in [min, max).
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}
