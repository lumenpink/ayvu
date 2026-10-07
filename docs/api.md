# Ayvu HTTP API Specification

> **Joaju-Compatible Microservice Architecture**
> Local daemon providing RESTful endpoints for real-time comic strip rendering.

---

## 1. Overview

Ayvu runs as an HTTP daemon listening on `127.0.0.1:3100` by default. It provides instant vector compilation of conversational scripts with CORS enabled for Obsidian, Fuxico, and web integrations.

### Starting the Daemon
```bash
ayvu serve --host 127.0.0.1 --port 3100
```

---

## 2. Endpoints

### `POST /render` (or `/comic`)
Compiles conversational text into a standalone SVG comic strip.

**Request Formats Supported:**
1. **JSON Payload:**
   ```json
   {
     "text": "Alice (happy): Good morning!\nBob (thinking): Is it morning already?",
     "title": "Breakfast Chat"
   }
   ```
2. **Raw Plain Text:**
   `Content-Type: text/plain` with raw dialogue markup in the request body.

**Response:**
- `Content-Type: image/svg+xml; charset=utf-8`
- Body: Standalone, responsive `<svg>...</svg>` document.

---

### `GET /render?text=...&title=...`
Allows direct image embedding in Markdown documents, forum posts, or web pages without JavaScript.

**Query Parameters:**
- `text`: URL-encoded dialogue markup script.
- `title` *(optional)*: URL-encoded title for the strip.

**Obsidian / Markdown Usage:**
```markdown
![Morning Strip](http://127.0.0.1:3100/render?text=Alice%20(happy):%20Hi!&title=Greeting)
```

**Response:**
- `Content-Type: image/svg+xml; charset=utf-8`

---

### `GET /health`
Liveness and health status check.

**Response (HTTP 200):**
```json
{
  "status": "ok",
  "service": "ayvu"
}
```

---

### `GET /version`
Returns current running version metadata.

**Response (HTTP 200):**
```json
{
  "version": "0.1.0-dev.1"
}
```

### `GET /characters`
Returns metadata for all available character packs in the registry.

**Response (HTTP 200):**
```json
[
  {
    "id": "arti",
    "name": "Arti",
    "author": "Lumen Pink",
    "license": "CC-BY",
    "description": "Arti, o acadêmico misterioso de sobretudo preto, óculos escuros e cabelos longos",
    "emotions": ["angry", "happy", "laughing", "neutral", "sad", "scared", "shouting", "thinking"]
  },
  {
    "id": "armando",
    "name": "Armando",
    "author": "Jim Woodring",
    "license": "MIT",
    "description": "Bohemian beatnik artist with beret, goatee and dark shades",
    "emotions": ["angry", "happy", "laughing", "neutral", "sad", "shouting", "thinking"]
  }
]
```

---

## 3. CORS Support

All endpoints include CORS headers to allow cross-origin requests from web applications and browser extensions:
- `Access-Control-Allow-Origin: *`
- `Access-Control-Allow-Methods: GET, POST, OPTIONS`
- `Access-Control-Allow-Headers: Content-Type, Authorization`
