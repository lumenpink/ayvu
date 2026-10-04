# Microsoft Comic Chat (1996) Engine & Artwork Port

## 1. Overview & Heritage

This specification documents the authentic port of Microsoft Comic Chat (1996) layout algorithms and Jim Woodring's original artwork assets into **Ayvu**.

Microsoft Comic Chat was originally released by Microsoft Research in 1996, created by David Kurlander, Tim Skelly, and David H. Salesin, with all iconic character art and surreal backdrops illustrated by celebrated cartoonist **Jim Woodring**. The source code and original asset archives were released by Microsoft under the permissive **MIT License** (`microsoft/comic-chat`).

---

## 2. Artwork Pipeline

### 2.1 Extraction & Composition
- **Avatars (`.avb`):** Extracted using reverse-engineered chunk parser decoding `AK_NFACES2` (chunk `0x0a`) and `AK_NTORSOS2` (chunk `0x0b`).
- **Neck Alignment Formula:** Faces are anchored dynamically to torsos using the exact Comic Chat C++ formula (`CBodyDouble::GetBodyBox` in `bodycam.cpp`):
  $$\text{xOffset} = \text{torso.cx} + \text{face.cxDelta} - \text{face.cx}$$
  $$\text{yOffset} = \text{torso.cy} + \text{face.cyDelta} - \text{face.cy}$$
- **Solid Aura Mask:** Every character bitmap retains Woodring's original white silhouette nimbus mask ($\alpha = 255$, rgb = 255), completely punching out background lines when characters stand in front of detailed scenery.
- **Embedded Zero-Dependency Storage:** All character poses (`neutral`, `happy`, `sad`, `angry`, `scared`, `shouting`, `thinking`, `laughing`) are compiled directly into the binary as optimized Base64 PNG data URIs in [`src/assets.rs`](file:///home/lumen/src/ayvu/src/assets.rs).

### 2.2 Classic Avatars Included
- **Armando:** The beatnik avatar with beret, round sunglasses, goatee, and black turtleneck.
- **Dan:** The young man avatar with rolled-up sleeves and necktie.
- **Tiki:** The surreal carved wooden mask with arms crossed.
- **Tongtyed (TongueTyed):** The classic plump cartoon creature.

### 2.3 Backdrops (`.bgb`)
- **Pastoral:** Jim Woodring's signature garden staircase, shaded trees, striated wood, and clouds.
- **Room:** Surreal interior with organic curved window looking out toward distant hills.
- **Field:** Classic Woodring open countryside landscape.

Directives in dialogue scripts:
```text
[backdrop: pastoral]
[backdrop: room]
[backdrop: field]
```

---

## 3. Layout Algorithm Port (`panel.cpp`)

1. **Avatar Proportions (`LayoutAvatars`):**
   - Characters are sized relative to the panel height ($H / 1.9 \approx 150\text{px}$).
   - Equidistant margins are calculated between panel borders and avatar bodies:
     $$\text{margin} = \frac{W_{\text{panel}} - \sum W_{\text{avatars}}}{N_{\text{speakers}} + 1}$$
   - Speakers are anchored physically to `floor_y` with soft contact shadows (when no backdrop is present).

2. **Balloon Placement & Anti-Crossing (`LayoutBalloons`):**
   - Left-to-right speakers have their balloons assigned to the upper-left and upper-right quadrants respectively.
   - Pointers originate from the bottom of each balloon and descend at a clean angle toward the speaker's mouth anchor (`arrowX`).
   - Tails never cross each other, adhering to Comic Chat's golden comic strip rule.

3. **Typography:**
   - Bold Comic Sans MS font family.
   - Dialogue text is rendered in all-caps via `style="text-transform: uppercase;"`.
