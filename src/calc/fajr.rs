use crate::calc::angles::{to_radians, to_degrees};
use crate::calc::solar::SolarPosition;

/// Calculate the hour angle for a given altitude and declination
/// Source: Astronomical Algorithms - Jean Meeus
pub fn calculate_hour_angle(latitude: f64, declination: f64, altitude: f64) -> Option<f64> {
    let lat_rad = to_radians(latitude);
    let alt_rad = to_radians(altitude);

    let cos_h = (alt_rad.sin() - lat_rad.sin() * declination.sin()) / (lat_rad.cos() * declination.cos());

    if cos_h < -1.0 || cos_h > 1.0 {
        return None; // Sun doesn't reach this altitude
    }

    Some(to_degrees(cos_h.acos()))
}

/// Calculate the time of day given solar position and hour angle
/// Result in hours from midnight
pub fn calculate_time(hour_angle: f64, equation_of_time: f64, longitude: f64, timezone: f64) -> f64 {
    // Standard time of solar noon
    let solar_noon = 12.0 + timezone - longitude / 15.0 - equation_of_time / 60.0;
    
    // Time relative to solar noon
    let time_diff = hour_angle / 15.0;
    
    solar_noon + time_diff
}

/// Helper to format hours into HH:MM
pub fn format_time(hours: f64) -> String {
    let mut normalized_hours = hours % 24.0;
    if normalized_hours < 0.0 {
        normalized_hours += 24.0;
    }

    let h = normalized_hours.floor() as i32;
    let m = ((normalized_hours - normalized_hours.floor()) * 60.0).round() as i32;
    
    let mut final_m = m;
    let mut final_h = h;
    
    if final_m == 60 {
        final_m = 0;
        final_h += 1;
    }
    
    format!("{:02}:{:02}", final_h % 24, final_m)
}
