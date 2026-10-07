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
  },
  {
    "id": "arti",
    "name": "Arti",
    "author": "Lumen Pink",
    "license": "CC-BY",
    "description": "Arti, o acadêmico misterioso de sobretudo preto, óculos escuros e cabelos longos",
    "emotions": [
      "angry",
      "happy",
      "laughing",
      "neutral",
      "sad",
      "scared",
      "shouting",
      "thinking"
    ]
  }
]
```

---

## 6. Official Character Roster (32 Characters)

Ayvu includes 32 bundled characters ready for instant dialogue rendering:

| Category | Characters |
| :--- | :--- |
| **Official Comic Chat (Jim Woodring, 1996)** | `anna`, `armando`, `bolo`, `buck`, `connor`, `cro`, `dan`, `denise`, `glenda`, `hugh`, `jordan`, `kevin`, `kirby`, `kwensa`, `lance`, `lynnea`, `margaret`, `maynard`, `mike`, `pedagog`, `rainbow`, `rebecca`, `sage`, `scotty`, `susan`, `tiki`, `tongtyed`, `tux`, `veronica`, `waf`, `xeno` (31 characters) |
| **Ayvu Original Roster** | `arti` (Arti, o acadêmico misterioso de sobretudo preto e óculos escuros, 160×440 px, 8 expressões canônicas completas) |

### Character Spotlight: Arti

* **Slug / ID**: `arti`
* **Author**: Lumen Pink
* **License**: CC-BY
* **Native Size**: 160 px wide × 440 px high
* **Speech Anchor**: `x: 0.50, y: 0.22` (mouth area right below dark shades)
* **8 Canonical Expressions**:
  1. `neutral`: Postura ereta, sobretudo fechado, olhar enigmático.
  2. `happy`: Sorriso amigável e acolhedor, mantendo o charme acadêmico.
  3. `shouting`: Boca aberta exclamando no microfone ou em debate acalorado.
  4. `thinking`: Braço apoiado, reflexão profunda sobre teorias e código.
  5. `angry`: Sobrancelhas franzidas, postura tensa e indignada.
  6. `sad`: Cabeça ligeiramente inclinada, expressão melancólica.
  7. `laughing`: Gargalhada aberta e espontânea.
  8. `scared`: Expressão assustada/surpresa, postura recuada.

---

## 7. Character Inspector & Absolute Ruler (`site/character.html`)

Ayvu provides a dedicated web-based **Character Inspector**:

* **Régua Fixa de Altura**: Régua milimétrica com escala vertical absoluta de 0 a 500 px e linha de chão fixa (`Y = 0 px`). Permite comparar proporções anatômicas reais entre personagens (ex.: Arti com 440 px vs. Armando com 210 px vs. Kirby com 160 px).
* **Comparação Multi-Personagem (Lineup)**: Seleção simultânea de personagens para avaliar silhuetas lado a lado.
* **Projeção em Cenários Vetoriais**: Teste de enquadramento sobre os cenários oficiais da UFSC (`praca_cidadania`, `ctc`, `cfh`, `pantanal`, `ru`, `bu_ufsc`, etc.).
* **Exportação & Compartilhamento**:
  - `🎨 SVG Lineup`: Exporta vetor SVG completo e autônomo com a régua e personagens.
  - `📥 PNG Lineup`: Exporta imagem PNG nítida em resolução Retina (2x).
  - `🔗 Copiar Link`: Compartilha a configuração exata via URL parameters (`?id=arti&compare=dan,armando&backdrop=praca_cidadania`).
* **Galeria de Variações em Alta Resolução**: Grade com zoom de até 2.2x e alternância de fundos (quadriculado de transparência, branco papel e preto nanquim) para auditoria fina de traço e recorte.

