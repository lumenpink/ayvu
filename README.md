# Ayvu (a-i-VÚ)

> **Ayvu** (*Guarani*): Human language, speech, dialogue, sacred word.
>
> Declarative comic strip compiler and microservice inspired by the classic Microsoft Comic Chat (1996).

---

## 📌 Overview

**Ayvu** transforms simple conversational text into expressive multi-panel comic strips. Operating as both a high-performance CLI utility and a standalone Joaju-style microservice, Ayvu provides:

1. **Declarative Dialogue Syntax**: Simple `Speaker (emotion): Speech` plain text format with automatic emotion and balloon heuristics.
2. **Dynamic Vector Compositor**: Pure SVG output with organic Bézier speech bubbles, cloud thought bubbles, jagged shout balloons, and character placement.
3. **Joaju-Style Microservice**: Minimal HTTP API (`POST /render` and `GET /render?text=...`) for embedding comics in Obsidian, Fuxico, and web applications.
4. **Zero-Dependency Native Binary**: Pure Rust compiled statically for instant execution (<5ms) and low memory usage.

---

## ⚡ Quickstart

### 1. Dialogue Markup Example (`strip.txt`)
```text
Alice (happy): Hey Bob, did you see the new release?
Bob (thinking): Should I tell her about the bugs?
Bob (laughing): Yes! It works like magic!
Alice (shouting): To the moon!
```

### 2. Compile to SVG
```bash
ayvu render strip.txt -o strip.svg
```

### 3. Run as HTTP Daemon
```bash
ayvu serve --port 3100
```

---

## 📖 Dialogue Markup Guide

See the full specification in [docs/markup.md](docs/markup.md).

---

## 📋 GOST Compliance

This project complies strictly with the **[GOST](https://codeberg.org/lumenpink/gost)** technical standard (Gosplan Standard for Technical Software).
- **GOST Version:** `1.1.0`
- **Languages:** `rust`
- **License:** `GPL-3.0`

```bash
cccp audit
cargo test
```

---

## 📄 License

Distributed under the [GNU General Public License v3.0 (GPL-3.0)](./LICENSE).
