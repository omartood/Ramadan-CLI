use crate::calc::angles::{to_radians, to_degrees};
use crate::models::types::Madhab;

/// Calculate the solar altitude for Asr
/// Source: Astronomical Algorithms - Jean Meeus
pub fn calculate_asr_altitude(madhab: Madhab, latitude: f64, declination: f64) -> f64 {
    let lat_rad = to_radians(latitude);
    
    // Shadow ratio: 1 for Shafi/others, 2 for Hanafi
    let shadow_ratio = match madhab {
        Madhab::Shafi => 1.0,
        Madhab::Hanafi => 2.0,
    };

    let decl_rad = declination; // Declination is already in radians from solar.rs
    
    // Altitude = arccot(shadow_ratio + tan(abs(latitude - declination)))
    let alt_rad = (shadow_ratio + (lat_rad - decl_rad).abs().tan()).atan().recip().atan();
    
    to_degrees(alt_rad)
}
