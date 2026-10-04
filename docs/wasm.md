# WebAssembly (WASM) Architecture and Browser Integration

Ayvu can compile its core parsing and vector rendering pipeline to pure WebAssembly (`wasm32-unknown-unknown`), enabling client-side rendering inside web browsers with zero backend server dependencies.

---

## 1. Architectural Model

The WebAssembly target isolates the mathematical dialogue parser and SVG composition engine from operating system primitives:

```text
┌─────────────────────────────────────────────────────────────┐
│                 Browser Client (ayvu.2lp.in)                │
│                                                             │
│   ┌─────────────────────┐         ┌─────────────────────┐   │
│   │  site/index.html    │ ──────> │  site/pkg/ayvu.js   │   │
│   │  (Editor & UI)      │         │  (ES Module Glue)   │   │
│   └─────────────────────┘         └──────────┬──────────┘   │
│                                              │              │
│                                              ▼              │
│                                   ┌─────────────────────┐   │
│                                   │ site/pkg/ayvu_bg.wasm│  │
│                                   │ (Core Rust Engine)  │   │
│                                   └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

- **Zero Network Latency**: Rendering occurs instantaneously in the browser thread as the user types.
- **Offline Capable**: Once cached, strips can be composed entirely without an internet connection.
- **Embedded Assets**: Built-in character pose packs (Tuffy, Jordan, Peep) are embedded inside the `.wasm` binary.

---

## 2. Compilation Target

Ayvu uses `wasm-bindgen` with an optional `wasm` Cargo feature to preserve native binary lightness:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = { version = "0.2", optional = true }

[features]
default = []
wasm = ["wasm-bindgen"]
```

Building the Wasm bundle with `wasm-pack`:
```bash
wasm-pack build --target web --out-dir site/pkg --release -- --no-default-features --features wasm
```

This outputs:
- `site/pkg/ayvu.js`: JavaScript initialization module and TypeScript definitions.
- `site/pkg/ayvu_bg.wasm`: Optimized WebAssembly bytecode (~120KB compressed).

---

## 3. JavaScript Interface

The exported function matches the Joaju / REST API signatures:

```javascript
import init, { render_comic } from './pkg/ayvu.js';

// 1. Initialize WASM memory
await init();

// 2. Render script to standalone SVG string
const script = `
Tuffy (happy): Running directly in your browser!
Jordan (surprised): Without contacting any backend server?
---
Peep (laughing): Pure Rust WebAssembly! [heart]
`;

try {
  const svg = render_comic(script, "WASM Demo", "strip");
  document.getElementById("preview").innerHTML = svg;
} catch (err) {
  console.error("Rendering failed:", err);
}
```

### Signature
```typescript
function render_comic(script: string, title?: string, layout?: string): string;
```

---

## 4. Fallback Strategy in Web Playground

The web playground (`site/index.html`) implements a cascading execution strategy:
1. **Primary**: WebAssembly execution (`render_comic` in browser).
2. **Secondary / Local Daemon**: If WASM fails to initialize (e.g. restrictive CSP or ancient browser), it falls back to the local HTTP daemon at `http://127.0.0.1:3100/render` via `POST`.
