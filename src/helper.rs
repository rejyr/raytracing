/// Returns a random real in [0.0, 1.0).
pub fn random_f64() -> f64 {
    fastrand::f64()
}

/// Returns a random real in [min, max).
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}

/// Returns a random integer in [min, max]
pub fn random_i32_in_range(min: i32, max: i32) -> i32 {
    random_f64_in_range(min as f64, max as f64 + 1.0) as i32
}
