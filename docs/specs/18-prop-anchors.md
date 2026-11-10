# Especificação Técnica: Encaixe e Ancoragem Anatômica de Props (#18)

## 1. Contexto & Problema
No motor anterior, os adereços empunhados pelos personagens (`[prop: item@Personagem]`) utilizavam valores fixos e arbitrários em `src/renderer/svg.rs` (`0.78` na largura e `floor_y - 32.0` no eixo vertical), desvinculados da geometria dos personagens e do centro geométrico de cada objeto.
Isso causava flutuação no ar ou sobreposição incorreta sobre as pernas/tronco.

## 2. Árvore de Decisão
- **Alternativa Descartada (Rigging / Esqueleto 2D):** Descartado por complexidade desnecessária (*overengineering*) para tirinhas estáticas em nanquim.
- **Alternativa Descartada (Manter números mágicos no renderer):** Descartado por quebrar a anatomia visual e dificultar a adição de novos personagens.
- **Abordagem Escolhida (Âncoras Normalizadas + Ponto de Pega / Grip):**
  1. No personagem: definir `hand_anchor: Anchor { x, y }` (valores normalizados `[0.0, 1.0]`) no manifesto e struct `CharacterManifest` / `CharacterPack`.
  2. Suporte extensível a poses: permitir `pose_hand_anchors: HashMap<String, Anchor>` opcional para suportar futuras poses expressivas de corpo inteiro, com fallback transparente para `hand_anchor`.
  3. No prop: definir `grip_offset: (f64, f64)` em `src/props.rs` mapeando o ponto exato de empunhadura de cada objeto (ex.: alça da xícara, base do microfone, topo da pasta).

## 3. Contrato Matemático da Ancoragem
Dado um personagem na posição `(char_x, char_y)` com largura escalada `scaled_w` e altura escalada `scaled_h`:
- Se não invertido (`flipped = false`):
  $$\text{hand\_x} = \text{char\_x} + (\text{hand\_anchor.x} \times \text{scaled\_w})$$
- Se invertido (`flipped = true`):
  $$\text{hand\_x} = \text{char\_x} + ((1.0 - \text{hand\_anchor.x}) \times \text{scaled\_w})$$
- Altura da mão:
  $$\text{hand\_y} = \text{char\_y} + (\text{hand\_anchor.y} \times \text{scaled\_h})$$
- Posição final do prop:
  $$\text{prop\_x} = \text{hand\_x} - \text{grip.x}$$
  $$\text{prop\_y} = \text{hand\_y} - \text{grip.y}$$
