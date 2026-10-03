# Ayvu Dialogue Markup Specification

> **Ayvu** (*Guarani*): Human language, speech, dialogue.
> Declarative dialogue format for compiling conversational text into comic strips.

---

## 1. Syntax Overview

Ayvu scripts are written in plain text, with each line representing a character dialogue, action, or panel boundary.

### Basic Dialogue
```text
Speaker: Speech text
```

### Dialogue with Explicit Emotion
```text
Speaker (emotion): Speech text
```

### Examples
```text
Alice (happy): Hey Bob, did you see the new release?
Bob (thinking): Should I tell her about the bugs?
Bob (laughing): Yes! It works like magic!
Alice (shouting): To the moon!
```

---

## 2. Emotions Catalog

Ayvu maps emotions to character poses and expressions:

| Emotion | Keywords / Aliases | Default Balloon |
| :--- | :--- | :--- |
| `neutral` | `neutro`, `normal`, `default` | Standard speech balloon |
| `happy` | `alegre`, `feliz`, `smile` | Standard speech balloon |
| `laughing` | `rindo`, `gargalhada`, `lol` | Standard speech balloon |
| `sad` | `triste`, `down` | Standard speech balloon |
| `crying` | `chorando`, `pranto` | Standard speech balloon with sweat/tear marks |
| `angry` | `bravo`, `zangado`, `furious` | Sharp / emphasized border |
| `shouting` | `gritando`, `grito`, `shout` | Jagged / spiky shout balloon |
| `thinking` | `pensando`, `pensativo` | Cloud balloon with trailing bubbles |
| `skeptical` | `cético`, `duvidoso`, `sus` | Standard speech balloon |
| `surprised` | `surpreso`, `espantado`, `wow` | Standard speech balloon |

---

## 3. Automatic Emotion & Balloon Heuristics

When no explicit emotion is specified, Ayvu inspects the text using linguistic heuristics:

1. **Thoughts:**
   - Text starting with `oO` or `...` triggers `BalloonType::Thought` and `Emotion::Thinking`.
2. **Shouts:**
   - Text in ALL CAPS with `!` or ending with `!!!` triggers `BalloonType::Shout` and `Emotion::Shouting`.
3. **Emoticons:**
   - `:)`, `:-)`, `\o/` $\rightarrow$ `Emotion::Happy`
   - `:D`, `xD`, `haha`, `kkk` $\rightarrow$ `Emotion::Laughing`
   - `:(`, `:-(` $\rightarrow$ `Emotion::Sad`
   - `:'(` $\rightarrow$ `Emotion::Crying`
   - `>:(`, `x(` $\rightarrow$ `Emotion::Angry`
   - `???` $\rightarrow$ `Emotion::Surprised`

---

## 4. Panel Boundaries

By default, Ayvu packages 1 to 2 consecutive dialogues into each comic panel. You can explicitly split panels using:

```text
Alice (happy): First panel dialogue.
---
Bob (surprised): Second panel dialogue!
```

Or using declarative directives:
```text
[panel title="Arrival"]
Alice: Here we are.
```
