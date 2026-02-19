use std::f64::consts::PI;

pub const DEGREES_TO_RADIANS: f64 = PI / 180.0;
pub const RADIANS_TO_DEGREES: f64 = 180.0 / PI;

/// Normalize an angle to [0, 360) degrees
pub fn normalize_degrees(angle: f64) -> f64 {
    let mut a = angle % 360.0;
    if a < 0.0 {
        a += 360.0;
    }
    a
}

pub fn to_radians(degrees: f64) -> f64 {
    degrees * DEGREES_TO_RADIANS
}

pub fn to_degrees(radians: f64) -> f64 {
    radians * RADIANS_TO_DEGREES
}

/// Calculate the arithmetic mean of two angles in degrees
pub fn mean_angle(a1: f64, a2: f64) -> f64 {
    normalize_degrees(a1 + (a2 - a1) / 2.0)
}
