# Ramadan CLI Project Rules

## 1. Project Goal

Create a fast, offline‑first Rust CLI tool that calculates Sehar (Fajr) and Iftar (Maghrib) accurately using astronomical formulas, not hardcoded tables or permanent API dependency.

The tool must:

* Work without internet
* Support multiple calculation methods
* Support multiple madhabs
* Be deterministic and reproducible

---

## 2. Engineering Principles

### Accuracy First

Prayer times are a religious matter. We prioritize correctness over features.

Rules:

* Never approximate angles unless documented
* Never mix calculation methods silently
* Every formula must cite a source in comments
* Unit tests required for every calculation change

### Offline First

The CLI must function fully without network.

Allowed network usage:

* Optional city lookup
* Optional hijri date sync

Core prayer calculations must never depend on API.

### Deterministic Output

Given the same inputs (date, lat, lon, method, madhab), output must always be identical across machines.

---

## 3. Code Structure

```
src/
 ├─ main.rs        // CLI parsing only
 ├─ commands/      // next, today, month
 ├─ calc/          // astronomy math
 │   ├─ solar.rs
 │   ├─ angles.rs
 │   ├─ fajr.rs
 │   └─ asr.rs
 ├─ models/        // PrayerTimes struct
 ├─ config/        // user settings
 └─ tui/           // optional terminal UI
```

Rules:

* main.rs must not contain logic
* All math lives in calc/
* CLI layer never performs calculations

---

## 4. Calculation Rules

### Required Calculations

We must compute using solar position:

* Solar declination
* Equation of time
* Hour angle
* Solar altitude angle

We DO NOT:

* Subtract fixed minutes from sunrise
* Use country tables
* Use Wikipedia shortcuts

### Fajr & Isha Angles

Support at minimum:

* Muslim World League (18°)
* Umm Al‑Qura (18.5° fajr, fixed isha offset)
* Egyptian (19.5°)

### Asr Madhab

Support:

* Shafi: shadow ratio = 1
* Hanafi: shadow ratio = 2

---

## 5. Testing Policy

Every commit affecting calculation must:

* Compare output with known datasets
* Test at least 5 cities
* Test extreme latitudes

Example cities:

* Mecca
* Mogadishu
* Oslo
* Jakarta
* New York

Tolerance:

* Max error = ±2 minutes

---

## 6. CLI UX Rules

Commands must remain simple and stable.

Allowed commands:

```
ramadan next
ramadan today
ramadan month
ramadan config
```

Do not introduce breaking flags casually.
CLI is a long‑term interface contract.

---

## 7. Commit Convention

Format:

```
<type>: <short description>
```

Types:

* feat: new feature
* fix: bug fix
* calc: math change
* refactor: code cleanup
* docs: documentation only
* test: tests only

Example:

```
calc: correct solar declination formula
```

---

## 8. Documentation Rules

Every formula must include:

* Source reference
* Link to paper or book
* Units explanation

Bad example:

```
// magic formula
```

Good example:

```
// Solar declination
// Source: Astronomical Algorithms - Jean Meeus
// angle in radians
```

---

## 9. Versioning Policy

We use semantic versioning:

* Patch: bug fixes
* Minor: new non‑breaking features
* Major: calculation change affecting output

Any prayer time change requires a MAJOR version bump.

---

## 10. Non‑Goals

We are NOT building:

* A full fiqh engine
* A fatwa system
* A moon sighting predictor

This project calculates times only.
