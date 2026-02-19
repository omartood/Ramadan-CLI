# Ramadan CLI 🌙

**Ku soo dhawaaw.** A fast, offline-first CLI for Islamic prayer times (Salaada) with Somali labels and terminal themes.

[![crates.io](https://img.shields.io/crates/v/cli-ramadam.svg)](https://crates.io/crates/cli-ramadam)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Features

- **Offline-first** — No API keys or internet; times are calculated locally
- **Somali-friendly** — Prayer names and welcome message in Somali (e.g. *Salaada Maanta*, *Qorraxda*, *Cishaha*)
- **Themes** — Dark, light, minimal, and colorful terminal styles
- **Customization** — Bold headers, dim separators, config stored in one file
- **Multiple methods** — Muslim World League, Umm Al-Qura, Egyptian (calculation logic in place)

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

| Command | Description |
|--------|-------------|
| `ramadan today` | Show today’s prayer times (default: Mogadishu) |
| `ramadan next` | Show the next prayer time *(coming soon)* |
| `ramadan month` | Show this month’s prayer times *(coming soon)* |
| `ramadan config show` | Show current config (theme, paths) |
| `ramadan config set --theme <name>` | Set theme: `dark`, `light`, `minimal`, `colorful` |
| `ramadan config set --bold-headers false` | Turn off bold headers |
| `ramadan config set --dim-separators false` | Turn off dim separators |

### Example output

```
Ku soo dhawaaw Ramadan CLI 🌙

Salaada Maanta (Mogadishu):
Taariikhda: 2025-02-19
-----------------------------
Fajr:     05:23
Qorraxda: 06:38
Dhuhr:    12:35
Asr:      15:55
Maghrib:  18:28
Cishaha:  19:38
-----------------------------
```

---

## Configuration

Config is stored in:

- **Linux / macOS:** `~/.config/ramadan/config.toml`
- **Windows:** `%APPDATA%\ramadan\config.toml`

### Themes

| Theme | Style |
|-------|--------|
| `dark` | Cyan/green on dark background (default) |
| `light` | Blue/bold for light terminals |
| `minimal` | No colors, plain text |
| `colorful` | Orange and light blue accents |

### Example config

```toml
theme = "dark"
bold_headers = true
dim_separators = true
```

Change anytime with:

```bash
ramadan config set --theme light
ramadan config show
```

---

## Building from source

```bash
git clone https://github.com/omartood/cli-ramadam.git
cd cli-ramadam
cargo build --release
```

Binary: `target/release/ramadan`.

---

## License

MIT — see [LICENSE](LICENSE).

---

*Barakallahu feek.*
