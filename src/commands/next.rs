use chrono::{Datelike, Local, NaiveDate, NaiveTime};
use crate::calc;
use crate::config::settings::{AppConfig, style};
use crate::models::types::{CalculationMethod, Madhab};

const PRAYER_ORDER: [(&str, fn(&crate::models::types::PrayerTimes) -> &str); 6] = [
    ("Fajr", |t| &t.fajr),
    ("Qorraxda", |t| &t.sunrise),
    ("Dhuhr", |t| &t.dhuhr),
    ("Asr", |t| &t.asr),
    ("Maghrib", |t| &t.maghrib),
    ("Cishaha", |t| &t.isha),
];

pub fn run() {
    let cfg = AppConfig::load();
    let (location, _name) = cfg.get_location();
    let now = Local::now();
    let now_naive = now.naive_local();
    let method = CalculationMethod::MuslimWorldLeague;
    let madhab = Madhab::Shafi;

    let today = calc::calculate_prayer_times(
        now.year(),
        now.month(),
        now.day(),
        &location,
        method,
        madhab,
    );

    let today_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();

    for (name, get_time) in PRAYER_ORDER {
        let time_str = get_time(&today);
        if let Ok(t) = NaiveTime::parse_from_str(time_str, "%H:%M") {
            let prayer_dt = today_date.and_time(t);
            if prayer_dt > now_naive {
                let left = prayer_dt - now_naive;
                let hours = left.num_hours();
                let mins = (left.num_minutes() % 60).abs();
                let secs = (left.num_seconds() % 60).abs();
                println!("{}", style::title(&cfg, "Salaadda soo socota"));
                println!(
                    "{} {} {}",
                    style::label(&cfg, &format!("{name}:")),
                    style::value(&cfg, time_str),
                    style::value(&cfg, &format!("({} saac {} dq {} il)", hours, mins, secs))
                );
                return;
            }
        }
    }

    let tomorrow = today_date.succ_opt().unwrap();
    let tomorrow_times = calc::calculate_prayer_times(
        tomorrow.year(),
        tomorrow.month(),
        tomorrow.day(),
        &location,
        method,
        madhab,
    );
    let fajr_t = NaiveTime::parse_from_str(&tomorrow_times.fajr, "%H:%M").unwrap();
    let tomorrow_fajr = tomorrow.and_time(fajr_t);
    let left = tomorrow_fajr - now_naive;
    let hours = left.num_hours();
    let mins = (left.num_minutes() % 60).abs();
    let secs = (left.num_seconds() % 60).abs();
    println!("{}", style::title(&cfg, "Salaadda soo socota"));
    println!(
        "{} {} {}",
        style::label(&cfg, "Fajr (berrito):"),
        style::value(&cfg, &tomorrow_times.fajr),
        style::value(&cfg, &format!("({} saac {} dq {} il)", hours, mins, secs))
    );
}
