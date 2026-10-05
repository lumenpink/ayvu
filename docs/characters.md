# Character Registry and Manifest Specification

Ayvu incorporates a modular character system inspired by the extensible actor system of Microsoft Comic Chat (1996). Characters in Ayvu are defined as isolated pose packs containing vector artwork and metadata.

---

## 1. Directory Structure

Character packs are discovered at runtime from the `assets/characters/` directory or bundled directly into the static binary as zero-dependency embedded assets:

```text
assets/characters/
├── tiki/
│   ├── manifest.json
│   ├── neutral.svg
│   ├── happy.svg
│   ├── sad.svg
│   ├── angry.svg
│   ├── surprised.svg
│   ├── shouting.svg
│   ├── thinking.svg
│   ├── laughing.svg
│   └── whispering.svg
├── jordan/
│   └── ...
└── armando/
    └── ...
```

---

## 2. Character Manifest Schema (`manifest.json`)

Each character directory must contain a valid `manifest.json`:

```json
{
  "$schema": "https://raw.githubusercontent.com/lumenpink/ayvu/main/schemas/character-manifest.json",
  "id": "tiki",
  "name": "Tiki",
  "author": "Jim Woodring / Microsoft Research (1996)",
  "license": "MIT",
  "description": "Surreal multi-eyed spirit character in Polynesian-inspired straw skirt",
  "balloon_anchor": {
    "x": 0.5,
    "y": 0.25
  },
  "poses": {
    "neutral": "neutral.svg",
    "happy": "happy.svg",
    "sad": "sad.svg",
    "angry": "angry.svg",
    "surprised": "surprised.svg",
    "shouting": "shouting.svg",
    "thinking": "thinking.svg",
    "laughing": "laughing.svg",
    "whispering": "whispering.svg"
  }
}
```

### Manifest Fields

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | `string` | Unique slug/identifier (lowercase alphanumeric with hyphens). |
| `name` | `string` | Human-readable character display name. |
| `author` | `string` | Creator / artist attribution. |
| `license` | `string` | SPDX license identifier (e.g. `GPL-3.0-or-later`, `CC-BY-4.0`). |
| `description` | `string` | Short character bio or visual description. |
| `balloon_anchor` | `object` | Relative coordinates `(x, y)` in `[0.0, 1.0]` representing the mouth/origin for speech balloon pointers relative to the character bounding box (default: `width=90`, `height=110`). |
| `poses` | `map<string, string>` | Map of emotion keys (`neutral`, `happy`, `sad`, `angry`, `surprised`, `shouting`, `thinking`, `laughing`, `whispering`) to relative SVG filenames. |

---

## 3. Pose SVG Requirements

Each pose SVG file:
1. Must be valid XML/SVG fragments or documents with a `viewBox="0 0 100 120"`.
2. Can use `<path>`, `<circle>`, `<rect>`, `<polygon>`, and other standard SVG elements.
3. Ayvu strips outer `<svg>` wrappers and embeds the graphic into `<g class="character-pose" transform="...">` inside the target panel.
4. If an emotion pose is missing from the pack, Ayvu automatically falls back to `neutral.svg`.

---

## 4. Speaker Resolution Algorithm

When rendering a dialogue script:
1. **Explicit Match**: If a dialogue speaker matches a registered character's `id` or `name` (case-insensitive), that character pack is assigned to the speaker.
2. **Dynamic Fallback (Round-Robin)**: If speaker names are arbitrary (e.g., `"Alice"`, `"Bob"`, `"User1"`), Ayvu assigns available characters in deterministic order (the first unknown speaker gets the first default character, the second gets the second, etc.).
3. **Left/Right Staging**: In each panel:
   - Panel with 1 speaker: Character staged on the left or center.
   - Panel with 2 speakers: First speaker on the left (`x: 25..115`), second speaker on the right (`x: 165..255`).

---

## 5. REST API Discovery (`GET /characters`)

The HTTP daemon provides a registry introspection endpoint:

### Request
```http
GET /characters HTTP/1.1
Host: localhost:8080
Accept: application/json
```

### Response (`200 OK`)
```json
[
  {
    "id": "armando",
    "name": "Armando",
    "author": "Jim Woodring / Microsoft Research (1996)",
    "license": "MIT",
    "description": "Bohemian beatnik artist with beret, goatee and dark shades",
    "emotions": [
      "angry",
      "happy",
      "laughing",
      "neutral",
      "sad",
      "shouting",
      "thinking"
    ]
  },
  {
    "id": "dan",
    "name": "Dan",
    "author": "Jim Woodring / Microsoft Research (1996)",
    "license": "MIT",
    "description": "Sharp business gentleman in necktie with expressive grimaces",
    "emotions": [
      "angry",
      "happy",
      "laughing",
      "neutral",
      "sad",
      "shouting",
      "thinking"
    ]
  },
  {
    "id": "tiki",
    "name": "Tiki",
    "author": "Jim Woodring / Microsoft Research (1996)",
    "license": "MIT",
    "description": "Surreal multi-eyed spirit character in Polynesian straw skirt",
    "emotions": [
      "angry",
      "happy",
      "laughing",
      "neutral",
      "sad",
      "shouting",
      "thinking"
    ]
  }
]
```
