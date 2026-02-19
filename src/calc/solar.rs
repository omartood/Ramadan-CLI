use crate::calc::angles::{normalize_degrees, to_radians, to_degrees};

/// Solar position calculations
/// Source: Astronomical Algorithms - Jean Meeus

pub struct SolarPosition {
    pub declination: f64,
    pub equation_of_time: f64,
}

/// Calculate the solar position for a given Julian Day
pub fn calculate_solar_position(julian_day: f64) -> SolarPosition {
    let d = julian_day - 2451545.0; // Days from J2000.0

    // Mean anomaly of the Sun
    let g = normalize_degrees(357.529 + 0.98560028 * d);
    let g_rad = to_radians(g);

    // Mean longitude of the Sun
    let q = normalize_degrees(280.459 + 0.98564736 * d);

    // Geocentric apparent ecliptic longitude
    let lambda = normalize_degrees(q + 1.915 * g_rad.sin() + 0.020 * (2.0 * g_rad).sin());
    let lambda_rad = to_radians(lambda);

    // Obliquity of the ecliptic
    let epsilon = 23.439 - 0.00000036 * d;
    let epsilon_rad = to_radians(epsilon);

    // Solar declination
    let declination = (epsilon_rad.sin() * lambda_rad.sin()).asin();
    
    // Right ascension
    let alpha = (epsilon_rad.cos() * lambda_rad.sin()).atan2(lambda_rad.cos());
    
    // Equation of Time (in degrees)
    // E = q / 15 - alpha * 180 / (pi * 15)
    // We want it in minutes usually, but Meeus gives RA in degrees or time.
    // Simplifying: Equation of Time in minutes
    let e_time = q / 15.0 - (to_degrees(alpha) / 15.0);
    // Ensure e_time is in range [-20, 20] minutes roughly
    let mut e_time = e_time * 60.0;
    while e_time > 20.0 { e_time -= 1440.0; }
    while e_time < -20.0 { e_time += 1440.0; }

    SolarPosition {
        declination,
        equation_of_time: e_time,
    }
}

pub fn date_to_julian_day(year: i32, month: u32, day: u32) -> f64 {
    let y = if month <= 2 { year - 1 } else { year } as f64;
    let m = if month <= 2 { month + 12 } else { month } as f64;
    let d = day as f64;

    let a = (y / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + d + b - 1524.5
}
