# Ramadan CLI 🌙

**Ku soo dhawaaw.** A fast, offline-first CLI for Islamic prayer times (Salaada) with Somali labels and terminal themes.

[![crates.io](https://img.shields.io/crates/v/cli-ramadam.svg)](https://crates.io/crates/cli-ramadam)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Features

- **Offline-first** — No API keys or internet; times are calculated locally
- **Beautiful UI** — Stunning card-style output using box-drawing characters (╭─╮│╰─╯)
- **Prayer Icons** — Islamic emojis (🌅 ☀️ 🕐 🌤️ 🌇 ⭐) for each prayer time
- **Next prayer** — `ramadan next` shows a styled card with countdown and **progress bar**
- **Somali-friendly** — Prayer names and messages in Somali (e.g. _Salaada Maanta_, _Qorraxda_)
- **Themes** — Dark, light, minimal, and colorful terminal styles with gradient-like accents
- **Location** — Set your city via config (latitude, longitude, timezone, display name)
- **Multiple methods** — Muslim World League, Umm Al-Qura, Egyptian calculations

---

## Installation

### From crates.io (recommended)

```bash
cargo install cli-ramadam
```

Then run:

```bash
ramadan today
```

### From source

```bash
git clone https://github.com/omartood/cli-ramadam.git
cd cli-ramadam
cargo install --path .
```

**Requirements:** [Rust](https://rustup.rs) (stable).

---

## Usage

| Command                                                                                     | Description                                       |
| ------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `ramadan today`                                                                             | Show today’s prayer times in a beautiful card     |
| `ramadan next`                                                                              | Show the next prayer, countdown, and progress bar |
| `ramadan month`                                                                             | Show this month’s prayer times _(coming soon)_    |
| `ramadan config show`                                                                       | Show current settings in a styled card            |
| `ramadan config set --theme <name>`                                                         | Set theme: `dark`, `light`, `minimal`, `colorful` |
| `ramadan config set --latitude <n> --longitude <n> --timezone <n> [--location-name <name>]` | Set your location                                 |

### Example output

```
  ╭────────────────────────────────────────╮
  │     🌙 Ku soo dhawaaw Ramadan CLI    │
  ╰────────────────────────────────────────╯

  ╭────────────────────────────────────────╮
  │                                       │
  │    🕌 Salaada Maanta (Mogadishu)    │
  │    📅 Taariikhda:  2026-02-22       │
  │                                       │
  ├────────────────────────────────────────┤
  │                                       │
  │    🌅 Fajr       : 05:01              │
  │    ☀️  Qorraxda   : 06:10              │
  │    🕐 Dhuhr      : 12:12              │
  │    🌤️  Asr        : 14:51              │
  │    🌇 Maghrib    : 18:14              │
  │    ⭐ Cishaha    : 19:20              │
  │                                       │
  ╰────────────────────────────────────────╯
```

---

## Configuration

Config is stored in:

- **Linux / macOS:** `~/.config/ramadan/config.toml`
- **Windows:** `%APPDATA%\ramadan\config.toml`

### Themes

| Theme      | Style                                           |
| ---------- | ----------------------------------------------- |
| `dark`     | Cyan/green accents on dark background (default) |
| `light`    | Blue/bold for light terminals                   |
| `minimal`  | No colors, plain text                           |
| `colorful` | Vibrant orange, pink, and light blue accents    |

### Location

Prayer times use the location in config (default: Mogadishu). Set your own:

```bash
# Example: Hargeisa (latitude 9.5, longitude 44.0, UTC+3)
ramadan config set --latitude 9.5 --longitude 44.0 --timezone 3 --location-name "Hargeisa"

# Or set only the display name (keep current coordinates)
ramadan config set --location-name "My City"
```

### Example config

```toml
theme = "dark"
bold_headers = true
dim_separators = true

[location]
latitude = 2.0469
longitude = 45.3182
timezone = 3.0
name = "Mogadishu"
```

Change anytime with:

```bash
ramadan config set --theme light
ramadan config set --latitude 9.5 --longitude 44.0 --timezone 3 --location-name "Hargeisa"
ramadan config show
```

---

## Roadmap — Making it amazing

| Idea                | Status     | Description                                     |
| ------------------- | ---------- | ----------------------------------------------- |
| **Beautiful TUI**   | ✅ Done    | Card-style output with box-drawing and icons.   |
| **Next prayer bar** | ✅ Done    | `ramadan next` shows a visual progress bar.     |
| **Month view**      | ⏳ Planned | `ramadan month` — full table of prayer times.   |
| **Hijri date**      | ⏳ Planned | Show Islamic date next to Gregorian.            |
| **Qibla direction** | ⏳ Planned | Show compass angle for your location.           |
| **Preset cities**   | ⏳ Planned | Quick picks: Mogadishu, Hargeisa, Nairobi, etc. |

---

## Building from source

```bash
cargo build --release
```

Binary: `target/release/ramadan`.

---

## License

MIT — see [LICENSE](LICENSE).

---

_Barakallahu feek._
