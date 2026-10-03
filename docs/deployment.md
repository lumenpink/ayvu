# Deployment and Integration Guide

Ayvu is engineered as a zero-dependency, ultra-lightweight standalone binary in pure Rust. It can run as a local CLI tool, a background HTTP microservice daemon, or a containerized microservice.

---

## 1. Local Installation

### Prerequisites
- POSIX-compatible operating system (Linux, macOS, BSD).
- Rust 1.80+ toolchain (`cargo`).

### Building and Installing via Makefile
```bash
# Clone the repository
git clone https://codeberg.org/lumenpink/ayvu.git
cd ayvu

# Build release binary
make release

# Run tests
make test

# Install to /usr/local/bin (requires sudo or root)
sudo make install

# Alternatively, install to user directory:
make PREFIX=$HOME/.local install
```

### Installing via `install.sh`
```bash
./install.sh --prefix ~/.local
```

### Makefile Targets
| Target | Description |
| :--- | :--- |
| `make` / `make all` | Builds debug binary. |
| `make release` | Builds optimized static release binary. |
| `make test` | Runs the test suite. |
| `make lint` | Runs `cargo clippy` and `cargo fmt --check`. |
| `make install` | Installs `ayvu` binary to `$(PREFIX)/bin`. |
| `make uninstall` | Removes installed `ayvu` binary. |
| `make clean` | Removes build artifacts. |

---

## 2. Container Deployment

Ayvu provides a multi-stage Docker build producing a statically linked, minimal container image (~10MB) with an unprivileged system user.

### Building Container Image
```bash
docker build -t ayvu:latest .
```

### Running with Docker
```bash
docker run -d \
  --name ayvu \
  -p 3100:3100 \
  -v $(pwd)/assets/characters:/app/assets/characters:ro \
  ayvu:latest
```

### Running with Docker Compose
```bash
docker compose up -d
```

Verify service health:
```bash
curl http://localhost:3100/health
```

---

## 3. Obsidian Integration

Ayvu can be embedded in Obsidian notes to render dynamic comic strips directly from Markdown.

### Method A: Direct Image Markdown Embed
Because Ayvu serves SVG over HTTP with full CORS support and URL query parameter decoding, you can embed comic strips directly into any note:

```markdown
![Daily Standup Comic](http://localhost:3100/render?text=Alice+(happy):+Tests+are+passing!%0A---%0ABob+(laughing):+Ready+for+deployment!&title=Sprint+Review&layout=grid)
```

### Method B: Obsidian Dataview / Custom Codeblock
Using a community plugin or custom JavaScript block:

```javascript
```dataviewjs
const dialogue = `
Alice (thinking): Did we remember to update docs?
Bob (shock): Doing it right now! [sweat]
`;

const res = await requestUrl({
  url: 'http://localhost:3100/render',
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ text: dialogue, title: "Obsidian Sync", layout: "vertical" })
});

dv.el("div", res.text);
\```
```

---

## 4. IndieInABox & Fuxico Integration

Ayvu provides drop-in replacement functionality for conversational chat log exports:

```bash
# Generate daily log comic from an IRC/Matrix/Fuxico chat log
cat /var/log/fuxico/chat.log | ayvu render - -o daily_strip.svg --title "Today in Chat" --layout grid
```
