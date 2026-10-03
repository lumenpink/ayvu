# Strip Layouts and Comic Visual Effects

Ayvu supports responsive strip arrangements and comic visual effects (sound effects, onomatopoeia impact stars, and manga-style emotion stickers).

---

## 1. Strip Layout Variants

Ayvu provides three core layout modes:

| Layout Mode | Directive / Value | Panel Arrangement | Best For |
| :--- | :--- | :--- | :--- |
| **Horizontal** *(Default)* | `horizontal` or `strip` | Single row ($N \times 1$) | Classic newspaper strips, desktop web banners |
| **Square Grid** | `grid` or `2x2` | 2 columns $\times \lceil N/2 \rceil$ rows | Instagram feeds, social sharing, square screens |
| **Vertical** | `vertical` or `webtoon` | Single column ($1 \times N$) | Mobile web, Webtoon scrolling, Obsidian sidebar embeds |

### Specifying Layouts

1. **In Script Markup**:
   ```text
   [layout: grid]
   # Or: [layout: vertical] / [layout: horizontal]

   Alice (happy): Look at this layout!
   Bob (surprised): It adapted into a 2x2 grid!
   ---
   Alice (laughing): Much easier to read on a phone!
   ```

2. **Via CLI Flag**:
   ```bash
   ayvu render story.txt --layout grid -o story.svg
   ayvu render story.txt --layout vertical -o story.svg
   ```

3. **Via HTTP REST API**:
   - Query parameter: `GET /render?text=...&layout=grid`
   - JSON payload:
     ```json
     {
       "text": "Alice: Hi!\nBob: Hello!",
       "layout": "vertical"
     }
     ```

---

## 2. Onomatopoeia Impact Badges

Sound effects (SFX) can be placed in dialogue or as standalone sound lines:

```text
[POW!]
Alice (angry): Take that!
---
Bob (surprised): What was that?
[CRASH!]
---
[BOOM!]
```

Supported impact keywords:
- `[POW!]`, `[BOOM!]`, `[CRASH!]`, `[BANG!]`, `[WHAM!]`, `[ZAP!]`, `[SMACK!]`, `[THUMP!]`, `[KABOOM!]`, `[CLICK!]`
- Any custom impact sound in brackets ending with `!` (e.g. `[KAPOW!]`).

### Visual Representation
- Golden yellow / crimson red 16-point starburst polygon backdrop.
- Bold, comic typography tilted at dynamic angles (`-12deg` to `+15deg`).
- 3D drop-shadow offset with comic halftone or solid action borders.

---

## 3. Emotion Stickers

Emotion stickers provide iconic visual iconography placed adjacent to characters:

| Directive | Visual Symbol | Semantic Meaning |
| :--- | :--- | :--- |
| `[sweat]` | Large blue sweat drop $\text{💧}$ | Nervousness, embarrassment, relief |
| `[shock]` | Red zigzag alert marks $\text{⚡}$ | Sudden realization, shock, awe |
| `[heart]` | Floating pink heart $\text{💖}$ | Love, affection, delight |
| `[idea]` | Glowing bright lightbulb $\text{💡}$ | Inspiration, eureka moment |
| `[question]` | Bold quizzical mark $\text{❓}$ | Confusion, inquiry |

### Example Usage
```text
Alice (happy): I love this! [heart]
Bob (thinking): How does this work? [idea]
---
Alice (angry): Don't break it! [shock]
Bob (sad): Oops... [sweat]
```
