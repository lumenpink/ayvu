# Ayvu Vector Rendering Architecture

> **«DANDO FORMA VISUAL À FALA»**
> Pure vector geometric compositor for comic strips.

---

## 1. Composition Hierarchy

Ayvu models a comic strip as a hierarchical vector document:

```text
ComicStrip
 ├── Title Bar (optional)
 └── Panels (1..N arranged horizontally or in a grid)
      ├── Panel Frame (border, background fill)
      ├── Speech / Thought / Shout Balloons
      │    ├── Bubble geometry (rounded rectangle, cloud, jagged polygon)
      │    ├── Tail geometry (quadratic Bézier curve pointing to speaker)
      │    └── Multi-line text spans
      └── Character Avatars & Poses
           ├── Pose SVG sprite / vector graphic
           ├── Caption badge (Speaker Name + Emotion)
           └── Anchor coordinates (Mouth pointer target)
```

---

## 2. Geometric Balloon Engine (`src/balloon.rs`)

Balões de quadrinhos clássicos precisam de geometria expressiva:

### 1. Speech Balloon (`BalloonType::Speech`)
- Rounded rectangle with border radius $R \approx 10\text{px}$.
- Quadratic Bézier pointer tail `M x1 y1 Q cx cy target_x target_y ...` connecting the bottom/side of the bubble smoothly to the character's mouth position.

### 2. Thought Balloon (`BalloonType::Thought`)
- Cloud-shaped bubble composed of overlapping arcs or rounded pill contour.
- Trailing circular bubbles of decreasing radii ($8\text{px} \rightarrow 5\text{px} \rightarrow 3\text{px}$) pointing down to the character's head.

### 3. Shout Balloon (`BalloonType::Shout`)
- Jagged, starburst polygon with alternating inner and outer vertices wrapping the exclamation.

### 4. Whisper Balloon (`BalloonType::Whisper`)
- Dashed border (`stroke-dasharray="4,3"`) representing quiet or secretive speech.

---

## 3. SVG Strip Compositor (`src/renderer/svg.rs`)

- Standard panel dimensions: $280 \times 280\text{px}$ per panel (scalable via `viewBox`).
- Responsive SVG outputting scalable vector graphics with inline styles and zero external web fonts dependency (uses system comic fonts with fallback to sans-serif).
