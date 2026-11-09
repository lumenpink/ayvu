# Ayvu (a-i-VÚ)

> **Ayvu** (*Guarani*): Human language, speech, dialogue, sacred word.
> Declarative comic strip compiler and microservice inspired by Microsoft Comic Chat (1996).

## Overview
**Ayvu** transforms conversational text into expressive multi-panel SVG comic strips:
- **Declarative Syntax**: `Speaker (emotion): Speech` plain text format with emotion heuristics.
- **Dynamic Vector Compositor**: Pure SVG output with organic Bézier bubbles and character placement.
- **Joaju-Style Microservice**: Minimal HTTP API (`POST /render` and `GET /render?text=...`).
- **Zero-Dependency Native Binary**: Pure Rust compiled statically for instant execution (<5ms).

## Quickstart
```text
Alice (happy): Hey Bob, did you see the new release?
Bob (thinking): Should I tell her about the bugs?
Bob (laughing): Yes! It works like magic!
```

```bash
ayvu render strip.txt -o strip.svg
ayvu serve --port 3100
```

## Documentation
* [Dialogue Markup Guide](docs/markup.md)
* [Architecture & Engine Details](docs/architecture.md)
* [Technical Specs](docs/specs/)

## Standards & Compliance
Complies with [GOST 1.4.0](.gost/GOST.md) and Lumen implementation profile via `cccp`.

```bash
cccp check --strict
cargo test
```

## License
Distributed under the [GNU General Public License v3.0 (GPL-3.0)](./LICENSE).
