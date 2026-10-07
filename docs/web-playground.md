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

---

## 3. Client-Side WebAssembly (`site/pkg/`)

Ayvu compiles natively to WebAssembly via `wasm-pack build --target web --out-dir site/pkg`:
- Zero server roundtrips required for previewing comic strips.
- Direct memory-based SVG rendering in browser runtime.
- Functions exported:
  - `render_dialogue_wasm(script, title)`
  - `list_characters_json()`
  - `get_character_detail_json(id)`
  - `list_backdrops_json()`
  - `get_backdrop_svg(name, width, height)`

---

## 4. Character Inspector (`site/character.html`)

A dedicated inspection and scale verification tool for comic artists and developers:

1. **Fixed Absolute Ruler (0 to 500 px)**:
   - Fixed millimetric graduated ruler on the left.
   - Groundline pinned at `Y = 0 px` (`ground_y = 510 px`).
   - Visual comparison of anatomical scale variations between characters (e.g. Arti at 440 px vs. classic Woodring characters).

2. **Lineup Comparison**:
   - Compare multiple characters simultaneously side by side.
   - Speech balloon anchor target crosshairs.
   - Character height guide lines aligning directly with the ruler.

3. **Lineup Export & Sharing**:
   - **🎨 SVG Lineup**: Downloads a clean standalone vector SVG of the lineup on the ruler.
   - **📥 PNG Lineup**: Generates a 2x retina raster image PNG of the stage.
   - **🔗 Copiar Link**: Copies the URL with state query parameters (`?id=arti&compare=dan,armando&backdrop=praca_cidadania`).

4. **High-Resolution Pose Variations Grid**:
   - Up to 2.2x zoom magnification for ink detailing.
   - Contrast modes: Checkerboard transparency, white paper, and nanquim black.
   - Individual SVG copy and download actions for each pose.

