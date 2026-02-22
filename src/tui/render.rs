use colored::Colorize;
use crate::config::settings::{AppConfig, Theme};
use crate::models::types::PrayerTimes;

// ─── Box Drawing Characters ─────────────────────────────────────────────────

const TL: &str = "╭";  // top-left
const TR: &str = "╮";  // top-right
const BL: &str = "╰";  // bottom-left
const BR: &str = "╯";  // bottom-right
const H: &str  = "─";  // horizontal
const V: &str  = "│";  // vertical
const ML: &str = "├";  // mid-left
const MR: &str = "┤";  // mid-right

const BOX_WIDTH: usize = 40;

// ─── Prayer Icons ────────────────────────────────────────────────────────────

fn prayer_icon(name: &str) -> &'static str {
    match name {
        "Fajr"     => "🌅",
        "Qorraxda" => "☀️ ",
        "Dhuhr"    => "🕐",
        "Asr"      => "🌤️ ",
        "Maghrib"  => "🌇",
        "Cishaha"  => "⭐",
        _          => "🕌",
    }
}

// ─── Color Helpers ───────────────────────────────────────────────────────────

fn border_color(config: &AppConfig, s: &str) -> String {
    match config.theme {
        Theme::Dark     => s.truecolor(80, 200, 220).to_string(),
        Theme::Light    => s.truecolor(70, 130, 210).to_string(),
        Theme::Minimal  => s.to_string(),
        Theme::Colorful => s.truecolor(255, 150, 50).to_string(),
    }
}

fn header_color(config: &AppConfig, s: &str) -> String {
    match config.theme {
        Theme::Dark     => s.truecolor(120, 230, 255).bold().to_string(),
        Theme::Light    => s.truecolor(30, 90, 180).bold().to_string(),
        Theme::Minimal  => s.bold().to_string(),
        Theme::Colorful => s.truecolor(255, 200, 80).bold().to_string(),
    }
}

fn prayer_name_color(config: &AppConfig, name: &str) -> String {
    match config.theme {
        Theme::Dark => {
            match name {
                "Fajr"     => name.truecolor(180, 140, 255).to_string(),
                "Qorraxda" => name.truecolor(255, 200, 80).to_string(),
                "Dhuhr"    => name.truecolor(100, 220, 180).to_string(),
                "Asr"      => name.truecolor(255, 180, 100).to_string(),
                "Maghrib"  => name.truecolor(255, 130, 100).to_string(),
                "Cishaha"  => name.truecolor(140, 160, 255).to_string(),
                _          => name.white().to_string(),
            }
        }
        Theme::Light => {
            match name {
                "Fajr"     => name.truecolor(100, 60, 180).to_string(),
                "Qorraxda" => name.truecolor(200, 140, 0).to_string(),
                "Dhuhr"    => name.truecolor(0, 130, 100).to_string(),
                "Asr"      => name.truecolor(180, 100, 30).to_string(),
                "Maghrib"  => name.truecolor(200, 60, 40).to_string(),
                "Cishaha"  => name.truecolor(60, 80, 180).to_string(),
                _          => name.truecolor(30, 30, 30).to_string(),
            }
        }
        Theme::Minimal => name.to_string(),
        Theme::Colorful => {
            match name {
                "Fajr"     => name.truecolor(200, 160, 255).bold().to_string(),
                "Qorraxda" => name.truecolor(255, 220, 100).bold().to_string(),
                "Dhuhr"    => name.truecolor(100, 255, 200).bold().to_string(),
                "Asr"      => name.truecolor(255, 200, 120).bold().to_string(),
                "Maghrib"  => name.truecolor(255, 150, 120).bold().to_string(),
                "Cishaha"  => name.truecolor(160, 180, 255).bold().to_string(),
                _          => name.white().bold().to_string(),
            }
        }
    }
}

fn time_color(config: &AppConfig, s: &str) -> String {
    match config.theme {
        Theme::Dark     => s.truecolor(220, 220, 240).to_string(),
        Theme::Light    => s.truecolor(40, 40, 60).to_string(),
        Theme::Minimal  => s.to_string(),
        Theme::Colorful => s.truecolor(240, 240, 255).bold().to_string(),
    }
}

fn dim_color(config: &AppConfig, s: &str) -> String {
    match config.theme {
        Theme::Dark     => s.truecolor(100, 110, 130).to_string(),
        Theme::Light    => s.truecolor(140, 140, 160).to_string(),
        Theme::Minimal  => s.to_string(),
        Theme::Colorful => s.truecolor(150, 120, 80).to_string(),
    }
}

fn accent_color(config: &AppConfig, s: &str) -> String {
    match config.theme {
        Theme::Dark     => s.truecolor(80, 255, 180).to_string(),
        Theme::Light    => s.truecolor(30, 160, 100).to_string(),
        Theme::Minimal  => s.to_string(),
        Theme::Colorful => s.truecolor(255, 120, 200).bold().to_string(),
    }
}

// ─── Box Building Helpers ────────────────────────────────────────────────────

fn top_border(config: &AppConfig) -> String {
    let line = H.repeat(BOX_WIDTH);
    format!("  {}", border_color(config, &format!("{TL}{line}{TR}")))
}

fn bottom_border(config: &AppConfig) -> String {
    let line = H.repeat(BOX_WIDTH);
    format!("  {}", border_color(config, &format!("{BL}{line}{BR}")))
}

fn mid_border(config: &AppConfig) -> String {
    let line = H.repeat(BOX_WIDTH);
    format!("  {}", border_color(config, &format!("{ML}{line}{MR}")))
}

/// Pad a visible string to fit in the box (accounting for emoji widths)
fn box_line(config: &AppConfig, content: &str, visible_len: usize) -> String {
    let padding = if BOX_WIDTH > visible_len + 2 {
        BOX_WIDTH - visible_len - 2
    } else {
        0
    };
    let left = border_color(config, V);
    let right = border_color(config, V);
    format!("  {left} {content}{:w$}{right}", "", w = padding)
}

fn empty_line(config: &AppConfig) -> String {
    box_line(config, "", 0)
}

// ─── Welcome Banner ──────────────────────────────────────────────────────────

pub fn render_welcome(config: &AppConfig) -> String {
    let text = "🌙 Ku soo dhawaaw Ramadan CLI";
    let visible_len = 30; // approximate visible length
    let total_pad = if BOX_WIDTH > visible_len + 2 { BOX_WIDTH - visible_len - 2 } else { 0 };
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;

    let inner = format!(
        "{:lw$}{}{:rw$}",
        "", header_color(config, text), "",
        lw = left_pad, rw = right_pad
    );

    let left = border_color(config, V);
    let right = border_color(config, V);
    let center_line = format!("  {left} {inner}{right}");

    format!(
        "\n{}\n{}\n{}\n",
        top_border(config),
        center_line,
        bottom_border(config),
    )
}

// ─── Today Command ───────────────────────────────────────────────────────────

pub fn render_today(config: &AppConfig, location_name: &str, times: &PrayerTimes) -> String {
    let mut lines = Vec::new();

    lines.push(String::new());
    lines.push(top_border(config));
    lines.push(empty_line(config));

    // Title line: 🕌  Salaada Maanta (Location)
    let title_text = format!("🕌 Salaada Maanta ({})", location_name);
    let title_colored = header_color(config, &title_text);
    // emoji 🕌 takes ~2 cols + space + text
    let title_vis_len = 22 + location_name.len();
    let total_pad = if BOX_WIDTH > title_vis_len + 2 { BOX_WIDTH - title_vis_len - 2 } else { 0 };
    let lp = total_pad / 2;
    let rp = total_pad - lp;
    let left = border_color(config, V);
    let right = border_color(config, V);
    lines.push(format!("  {left} {:lw$}{title_colored}{:rw$}{right}", "", "", lw=lp, rw=rp));

    // Date line
    let date_label = dim_color(config, "📅 Taariikhda:");
    let date_val = time_color(config, &times.date);
    let date_content = format!("   {}  {}", date_label, date_val);
    let date_vis = 3 + 16 + 2 + times.date.len();
    lines.push(box_line(config, &date_content, date_vis));

    lines.push(empty_line(config));
    lines.push(mid_border(config));
    lines.push(empty_line(config));

    // Prayer lines
    let prayers: [(&str, &str); 6] = [
        ("Fajr", &times.fajr),
        ("Qorraxda", &times.sunrise),
        ("Dhuhr", &times.dhuhr),
        ("Asr", &times.asr),
        ("Maghrib", &times.maghrib),
        ("Cishaha", &times.isha),
    ];

    for (name, time) in &prayers {
        let icon = prayer_icon(name);
        let name_padded = format!("{:<10}", name);
        let name_col = prayer_name_color(config, &name_padded);
        let colon = dim_color(config, ":");
        let time_col = time_color(config, time);

        let content = format!("   {} {} {} {}", icon, name_col, colon, time_col);
        // visible: 3 + 2(icon) + 1 + 10 + 1(space) + 1(:) + 1 + 5(time) = ~24
        let vis = 3 + 2 + 1 + 10 + 1 + 1 + 1 + time.len();
        lines.push(box_line(config, &content, vis));
    }

    lines.push(empty_line(config));
    lines.push(bottom_border(config));
    lines.push(String::new());

    lines.join("\n")
}

// ─── Next Prayer Command ────────────────────────────────────────────────────

pub fn render_next(
    config: &AppConfig,
    prayer_name: &str,
    time_str: &str,
    hours: i64,
    mins: i64,
    secs: i64,
    is_tomorrow: bool,
) -> String {
    let mut lines = Vec::new();

    lines.push(String::new());
    lines.push(top_border(config));
    lines.push(empty_line(config));

    // Title
    let title = "⏳ Salaadda Soo Socota";
    let title_col = header_color(config, title);
    let title_vis = 23;
    let total_pad = if BOX_WIDTH > title_vis + 2 { BOX_WIDTH - title_vis - 2 } else { 0 };
    let lp = total_pad / 2;
    let rp = total_pad - lp;
    let left = border_color(config, V);
    let right = border_color(config, V);
    lines.push(format!("  {left} {:lw$}{title_col}{:rw$}{right}", "", "", lw=lp, rw=rp));

    lines.push(empty_line(config));
    lines.push(mid_border(config));
    lines.push(empty_line(config));

    // Prayer name and time
    let icon = prayer_icon(prayer_name);
    let display_name = if is_tomorrow {
        format!("{} (berrito)", prayer_name)
    } else {
        prayer_name.to_string()
    };
    let name_col = prayer_name_color(config, &display_name);
    let at_label = dim_color(config, "waqtiga:");
    let time_col = time_color(config, time_str);
    let prayer_content = format!("   {} {}  {} {}", icon, name_col, at_label, time_col);
    let prayer_vis = 3 + 2 + 1 + display_name.len() + 2 + 8 + 1 + time_str.len();
    lines.push(box_line(config, &prayer_content, prayer_vis));

    lines.push(empty_line(config));

    // Countdown
    let countdown_text = format!("{}h {}m {}s ka haray", hours, mins, secs);
    let countdown_col = accent_color(config, &countdown_text);
    let countdown_vis = countdown_text.len();
    let total_pad2 = if BOX_WIDTH > countdown_vis + 2 { BOX_WIDTH - countdown_vis - 2 } else { 0 };
    let lp2 = total_pad2 / 2;
    let rp2 = total_pad2 - lp2;
    lines.push(format!("  {left} {:lw$}{countdown_col}{:rw$}{right}", "", "", lw=lp2, rw=rp2));

    lines.push(empty_line(config));

    // Progress bar (visual countdown)
    let total_secs = hours * 3600 + mins * 60 + secs;
    let max_secs: i64 = 6 * 3600; // assume max ~6 hours between prayers
    let progress = if total_secs > max_secs {
        0.0
    } else {
        1.0 - (total_secs as f64 / max_secs as f64)
    };
    let bar_width = BOX_WIDTH - 8;
    let filled = (progress * bar_width as f64) as usize;
    let empty = bar_width - filled;
    let bar_filled = "█".repeat(filled);
    let bar_empty = "░".repeat(empty);
    let bar = format!("   {}{}", accent_color(config, &bar_filled), dim_color(config, &bar_empty));
    let bar_vis = 3 + bar_width;
    lines.push(box_line(config, &bar, bar_vis));

    lines.push(empty_line(config));
    lines.push(bottom_border(config));
    lines.push(String::new());

    lines.join("\n")
}

// ─── Config Show Command ────────────────────────────────────────────────────

pub fn render_config(
    config: &AppConfig,
    config_path: &str,
    location_name: &str,
    lat: f64,
    lon: f64,
    tz: f64,
) -> String {
    let mut lines = Vec::new();

    lines.push(String::new());
    lines.push(top_border(config));
    lines.push(empty_line(config));

    // Title
    let title = "⚙️  Qaabeynta (Config)";
    let title_col = header_color(config, title);
    let title_vis = 23;
    let total_pad = if BOX_WIDTH > title_vis + 2 { BOX_WIDTH - title_vis - 2 } else { 0 };
    let lp = total_pad / 2;
    let rp = total_pad - lp;
    let left = border_color(config, V);
    let right = border_color(config, V);
    lines.push(format!("  {left} {:lw$}{title_col}{:rw$}{right}", "", "", lw=lp, rw=rp));

    lines.push(empty_line(config));
    lines.push(mid_border(config));
    lines.push(empty_line(config));

    // Shorten config path: ~/.config/ramadan/config.toml -> ~/…/config.toml
    let short_path = if config_path.contains("config.toml") {
        "~/.config/ramadan/config.toml"
    } else {
        config_path
    };

    // Truncate any value to max_val_len chars
    let max_val_len = BOX_WIDTH - 18; // leaves room for icon+label+padding
    let truncate = |s: &str| -> String {
        if s.len() > max_val_len {
            format!("{}…", &s[..max_val_len - 1])
        } else {
            s.to_string()
        }
    };

    // Config items
    let location_str = format!("{} (UTC{:+})", location_name, tz);
    let items: Vec<(&str, String)> = vec![
        ("📁 Faylka",    truncate(short_path)),
        ("🎨 Theme",     format!("{:?}", config.theme).to_lowercase()),
        ("✏️  Bold",      if config.bold_headers { "haa ✓".to_string() } else { "maya ✗".to_string() }),
        ("🔅 Dim",       if config.dim_separators { "haa ✓".to_string() } else { "maya ✗".to_string() }),
        ("📍 Goobta",    truncate(&location_str)),
    ];

    for (label, val) in &items {
        let label_col = dim_color(config, &format!("{:<12}", label));
        let val_col = time_color(config, val);
        let content = format!("   {} {}", label_col, val_col);
        let vis = 3 + 12 + 1 + val.len();
        lines.push(box_line(config, &content, vis));
    }

    lines.push(empty_line(config));

    // Coordinates on separate line
    let coords_text = format!("lat:{} lon:{}", lat, lon);
    let coords_content = format!("        {}", dim_color(config, &coords_text));
    let coords_vis = 8 + coords_text.len();
    lines.push(box_line(config, &coords_content, coords_vis));

    lines.push(empty_line(config));

    // Theme options
    let themes_label = dim_color(config, "Themes:");
    let themes_val = "dark·light·minimal·colorful";
    let themes_list = accent_color(config, themes_val);
    let themes_content = format!("   {} {}", themes_label, themes_list);
    let themes_vis = 3 + 8 + 1 + themes_val.len();
    lines.push(box_line(config, &themes_content, themes_vis));

    lines.push(empty_line(config));
    lines.push(bottom_border(config));
    lines.push(String::new());

    lines.join("\n")
}
