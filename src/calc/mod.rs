pub mod solar;
pub mod angles;
pub mod fajr;
pub mod asr;

use solar::{calculate_solar_position, date_to_julian_day};
use fajr::{calculate_hour_angle, calculate_time, format_time};
use asr::calculate_asr_altitude;
use crate::models::types::{PrayerTimes, Location, Madhab, CalculationMethod};

pub fn calculate_prayer_times(
    year: i32,
    month: u32,
    day: u32,
    location: &Location,
    method: CalculationMethod,
    madhab: Madhab,
) -> PrayerTimes {
    let jd = date_to_julian_day(year, month, day);
    let solar = calculate_solar_position(jd);

    // Fajr Angle based on method
    let fajr_angle = match method {
        CalculationMethod::MuslimWorldLeague => -18.0,
        CalculationMethod::UmmAlQura => -18.5,
        CalculationMethod::Egyptian => -19.5,
    };

    // Isha Angle/Offset
    let isha_angle = match method {
        CalculationMethod::MuslimWorldLeague => -17.0,
        CalculationMethod::UmmAlQura => -18.5, // Umm Al-Qura uses fixed offset usually, but we'll use angle for now
        CalculationMethod::Egyptian => -17.5,
    };

    // Calculate times
    // Fajr
    let fajr_ha = calculate_hour_angle(location.latitude, solar.declination, fajr_angle).unwrap_or(0.0);
    let fajr_time = calculate_time(-fajr_ha, solar.equation_of_time, location.longitude, location.timezone);

    // Sunrise
    let sunrise_ha = calculate_hour_angle(location.latitude, solar.declination, -0.833).unwrap_or(0.0);
    let sunrise_time = calculate_time(-sunrise_ha, solar.equation_of_time, location.longitude, location.timezone);

    // Dhuhr
    let dhuhr_time = 12.0 + location.timezone - location.longitude / 15.0 - solar.equation_of_time / 60.0;

    // Asr
    let asr_alt = calculate_asr_altitude(madhab, location.latitude, solar.declination);
    let asr_ha = calculate_hour_angle(location.latitude, solar.declination, asr_alt).unwrap_or(0.0);
    let asr_time = calculate_time(asr_ha, solar.equation_of_time, location.longitude, location.timezone);

    // Maghrib
    let maghrib_ha = calculate_hour_angle(location.latitude, solar.declination, -0.833).unwrap_or(0.0);
    let maghrib_time = calculate_time(maghrib_ha, solar.equation_of_time, location.longitude, location.timezone);

    // Isha
    let isha_ha = calculate_hour_angle(location.latitude, solar.declination, isha_angle).unwrap_or(0.0);
    let isha_time = calculate_time(isha_ha, solar.equation_of_time, location.longitude, location.timezone);

    PrayerTimes {
        date: format!("{}-{:02}-{:02}", year, month, day),
        fajr: format_time(fajr_time),
        sunrise: format_time(sunrise_time),
        dhuhr: format_time(dhuhr_time),
        asr: format_time(asr_time),
        maghrib: format_time(maghrib_time),
        isha: format_time(isha_time),
    }
}
