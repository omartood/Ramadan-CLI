use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimes {
    pub date: String,
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
  pub maghrib: String,
    pub isha: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Madhab {
    Shafi,
    Hanafi,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CalculationMethod {
    MuslimWorldLeague,
    UmmAlQura,
    Egyptian,
}
