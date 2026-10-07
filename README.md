<div align="center">
  <img src="https://tuquet.github.io/icons/random-user-generator.svg" width="76" height="76" alt="Tuquet Faker Logo" />
  <h1>Tuquet Faker</h1>
  <p><strong>Enterprise-Grade Synthetic Persona &amp; Test Identity Generator in Rust</strong></p>

  <p>
    <a href="https://github.com/tuquet/scoop-bucket"><img src="https://img.shields.io/badge/Scoop-tuquet-blue.svg" alt="Scoop Bucket" /></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="Rust" /></a>
    <img src="https://img.shields.io/badge/SSOT-Pillar%205%20(~%2F.specter%2Ffaker)-purple.svg" alt="Pillar 5" />
    <a href="https://github.com/tuquet/skills/blob/main/skills/tuquet-faker/SKILL.md"><img src="https://img.shields.io/badge/Skill-%2Ftuquet--faker-purple.svg" alt="Tuquet Faker Skill" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" /></a>
  </p>
  <p><strong><a href="https://github.com/tuquet/skills/blob/main/skills/tuquet-faker/SKILL.md">⚡ Operational Skill Reference (`/tuquet-faker`) &rarr;</a></strong></p>
</div>

---

## Executive Summary

**Tuquet Faker** (`tuquet-faker`) is a high-performance, offline-first synthetic persona and identity generation engine for software developers, QA automation suites, and anti-detect multi-profile environments.

Engineered natively in pure Rust with zero webview overhead, it produces realistic, demographically weighted, and structurally validated identities with compliant Vietnamese Citizen Identity numbers (CCCD), cohesive address hierarchies, and high-entropy credentials in sub-millisecond execution time.

It functions both as a **standalone CLI binary** and as a **modular plugin/crate** for the [Tuquet Master CLI](https://github.com/tuquet/cli).

---

## Key Capabilities

* **Regulatory-Compliant National Identification (CCCD)**:
  * Deterministic 12-digit encoding: Province code (3 digits) + Century/Gender (1 digit) + Birth year suffix (2 digits) + Unique sequence (6 digits).
* **Geographically Cohesive Address Mapping**:
  * Real hierarchical mapping: `Province / City` $\longrightarrow$ `District` $\longrightarrow$ `Ward` $\longrightarrow$ `Street Name`, matching official postcodes.
* **Demographically Weighted Demographics**:
  * Vietnamese surnames accurately weighted based on population statistics (Nguyen ~38.4%, Tran ~11.0%, Le ~9.5%, Pham ~7.1%, etc.).
* **Enterprise High-Entropy Credentials**:
  * Passwords with guaranteed uppercase, lowercase, numbers, and symbols.
  * Clean RFC 5322 ASCII-only emails without diacritics.
* **Sub-Millisecond 0ms Latency**:
  * Completely offline, air-gapped data generation without any external network round-trip.
* **Flexible Export Options**:
  * Terminal interactive table, single-persona card audit, or batch export to JSON/CSV.

---

## 🏛️ SSOT Configuration & Storage Architecture (Pillar 5)

In strict adherence to Tuquet's **Single Source of Truth (SSOT)** and microservice pillar standards, all Faker schema configurations, override rules, and custom templates resolve exclusively to the canonical root domain `~/.specter/faker/`:

```text
~/.specter/faker/
├── faker.json                # Schema override configuration & email domain pool
└── templates/                # Custom demographic templates & localized identity weights
```

### 1. Schema & Domain Override (`~/.specter/faker/faker.json`)
The `faker.json` configuration file controls email domain rotation, pattern templates, default locale, and avatar styles across all CLI and REPL executions:

```json
{
  "email_domains": [
    "flowup.io.vn",
    "tuquet.dev",
    "enterprise.internal"
  ],
  "default_domain": "flowup.io.vn",
  "email_pattern": "first.last",
  "default_nat": "VN",
  "default_avatar": "real"
}
```

Manage and inspect configuration directly via Master CLI:
```powershell
# Display active SSOT configuration card
tuquet faker config --show

# Open faker.json in default system editor
tuquet faker config --edit

# Dynamically set or prepend a default email domain
tuquet faker config -d yourcompany.com
```

### 2. Custom Demographic Templates (`~/.specter/faker/templates/`)
Store custom demographic datasets and template overrides inside `~/.specter/faker/templates/`:
- **Demographic Weights**: Customize surname and given-name distribution weights for specialized regional testing.
- **Address & Postcode Hierarchies**: Override provincial, district, or ward postal mappings.
- **Deterministic Identity Formats**: Provide custom series patterns for specialized enterprise test scenarios.

---

## ⚡ Master CLI & Interactive REPL Integration

Tuquet Faker integrates directly into the unified Tuquet developer experience via both direct CLI execution and the interactive REPL shell:

### 1. Direct Command (`tuquet faker generate`)
Generate realistic, verified synthetic identities directly from PowerShell or Command Prompt:

```powershell
# Generate 5 Vietnamese profiles in interactive terminal table
tuquet faker generate -n 5

# Generate 10 profiles in JSON format
tuquet faker generate -n 10 -f json -o users.json

# Export 100 profiles to CSV with gender and nationality filters
tuquet faker generate -n 100 --gender female --nat VN -f csv -o test_users.csv

# Render a detailed verified identity card in terminal
tuquet faker card --gender female
```

### 2. Master CLI REPL (`tuquet use faker`)
Inside the Tuquet Master CLI interactive shell, switch directly to the Faker scope using `use faker` for rapid, continuous profile generation:

```console
# Launch interactive REPL
tuquet
```

```text
============================================================
  🛸 Tuquet Unified Interactive Shell (v1.0.0)
============================================================
Type 'help' for commands, 'use <service>' to switch scope, 'exit' to quit.

tuquet> use faker
tuquet(faker)> generate -n 5
tuquet(faker)> card --gender female
tuquet(faker)> config --show
tuquet(faker)> back
tuquet> exit
```

> 💡 **Direct Scope Shortcut**: You can also enter the Faker scope directly from terminal via `tuquet shell faker`.

---

## 🛠️ Quick Start (Standalone CLI)

### 1. Build & Run
```powershell
# Generate 5 Vietnamese profiles in formatted terminal table
cargo run -- -n 5

# Generate profiles as JSON
cargo run -- -n 3 -f json

# Export 100 profiles to CSV file
cargo run -- -n 100 -f csv -o test_users.csv

# Filter by gender and nationality
cargo run -- -n 10 --gender female --nat VN
```

### 2. Options Reference
```text
Options:
  -n, --count <COUNT>      Number of profiles to generate [default: 1]
  -g, --gender <GENDER>    Gender filter: male, female, or all [default: all]
      --nat <NAT>          Nationality filter: VN, US, or all [default: VN]
      --avatar <AVATAR>    Avatar style: real or svg [default: real]
  -f, --format <FORMAT>    Output format: table, json, or csv [default: table]
  -o, --output <OUTPUT>    Optional file path to save output
  -h, --help               Print help
  -V, --version            Print version
```

---

## Rust Library API

Add to your `Cargo.toml`:
```toml
[dependencies]
tuquet-faker = { path = "../faker" }
```

Usage in code:
```rust
use tuquet_faker::{generate_users, to_csv, to_json};

fn main() {
    // Generate 10 offline profiles
    let data = generate_users(10, Some("female"), Some("VN"), Some("real"));
    let users = data["results"].as_array().unwrap();

    // Export to RFC-compliant CSV
    let csv = to_csv(users);
    println!("{}", csv);
}
```

---

## License

Distributed under the **MIT License**. Maintained by **Tuquet Team**.
