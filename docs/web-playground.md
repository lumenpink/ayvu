# Ayvu Web Playground & Deployment Specification

The Ayvu Web Playground is a modern, single-page application and interactive documentation portal designed for both browser-based exploration and edge deployment.

---

## 1. Architectural Architecture

The web interface is decoupled into two complementary operational modes:

1. **Embedded Daemon Mode (`ayvu serve`)**:
   - The compiled static binary embeds the web playground assets via `include_str!`.
   - Accessing `http://localhost:3100/` or `http://127.0.0.1:3100/` immediately serves the full playground interface with zero external dependencies.
   - The editor dispatches `POST /render` calls directly to the local HTTP daemon.

2. **Edge Vitrine Mode (`ayvu.2lp.in`)**:
   - Published via Cloudflare Workers / Static Assets under the unified `*.2lp.in` domain.
   - Configured via `wrangler.jsonc` and `site/_headers`.
   - Can render using either the user's local daemon (`http://localhost:3100`), a custom server endpoint, or client-side SVG composition.

---

## 2. Interface Capabilities & Cheatsheet

### A. Editor & Preset Selector
- **Interactive Script Editor**: Live typing area with sample presets:
  - *Classic Duo*: Retro conversation between Tuffy and Jordan.
  - *Action Impact*: Comic brawl showcasing `[POW!]`, `[CRASH!]`, and shout balloons.
  - *Emotions & Stickers*: Demonstrating `[heart]`, `[sweat]`, `[shock]`, and `[idea]`.
  - *Grid Layout*: 4-panel social media square 2x2.
  - *Webtoon Scrolling*: Single-column vertical strip.

### B. Quick-Insert Toolbar
One-click insertion chips for:
- **Characters**: `Tuffy`, `Jordan`, `Peep`.
- **Emotions**: `(happy)`, `(angry)`, `(sad)`, `(thinking)`, `(laughing)`, `(surprised)`.
- **Onomatopoeia**: `[POW!]`, `[BOOM!]`, `[CRASH!]`, `[ZAP!]`, `[BANG!]`.
- **Stickers**: `[heart]`, `[sweat]`, `[shock]`, `[idea]`, `[question]`.
- **Layouts**: `[layout: strip]`, `[layout: grid]`, `[layout: vertical]`.

### C. Live Preview & Export
- SVG rendered dynamically in a high-contrast canvas.
- **Download SVG**: Saves `comic.svg` to local machine.
- **Copy SVG**: Copies raw XML directly to clipboard.
- **Obsidian / Markdown Link**: Generates a pre-formatted image URL for direct paste into Obsidian notes or Markdown blogs.

### D. Integrated Cheatsheet
An interactive drawer/tab providing immediate reference for:
- Syntax grammar (`Speaker (emotion): dialogue`).
- Emoticon shortcuts (`:)`, `:D`, `XD`, `:(`, `oO`, `!`).
- REST API usage with `curl` examples.
- Obsidian Dataview JS snippet.
