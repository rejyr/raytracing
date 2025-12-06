/// Returns a random real in [0.0, 1.0).
pub fn random_f64() -> f64 {
    fastrand::f64()
}

/// Returns a random real in [min, max).
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}
