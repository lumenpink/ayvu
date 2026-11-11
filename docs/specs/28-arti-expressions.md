# Especificação Técnica — Diferenciação das 8 Poses do Arti (#28)

- **ID do Ticket:** #28
- **Status:** Em Implementação
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Comic Chat (Jim Woodring)

## 1. Contexto & Árvore de Decisão
Na reestruturação de assets do Ticket #22 e no commit `8f3d610`, o personagem **Arti** (o acadêmico de sobretudo longo, cabelos compridos, óculos e calça saruel) teve seus 8 arquivos vetoriais sobrescritos com exatamente a mesma imagem estática da pose neutra.
Essa repetição quebra a expressividade visual das tirinhas, onde o motor do Comic Chat altera a fisionomia e gestualidade dos personagens de acordo com a emoção (`happy`, `sad`, `shouting`, `thinking`, `angry`, `scared`, `laughing`).

**Decisão para a Natasha:**
Manter a anatomia canônica de sucesso do Arti em nanquim de alto contraste, mas desenhar e diferenciar fisionomicamente cada uma das 8 poses:
- Olhos através dos óculos (pupilas, aberturas, piscar de olhos, sobrancelhas).
- Boca sob o cavanhaque e bigode (fechada, sorriso, riso aberto com dentes, boca em projeção de grito, lábios apertados).
- Gestos e variações de cabeça/postura de sobretudo em nanquim autêntico.

## 2. Detalhamento das 8 Poses Canônicas
1. **neutral:** Postura em pé de sobretudo, óculos neutros, cavanhaque fechado e mãos relaxadas.
2. **happy:** Olhos arqueados em semicírculo alegre por trás das lentes, cantos do lábio erguidos em sorriso acolhedor.
3. **laughing:** Gargalhada aberta com dentes superiores visíveis no centro do cavanhaque, olhos fechados felizes em arco.
4. **shouting:** Boca aberta em projeção de fala/grito, sobrancelhas arqueadas, olhar firme.
5. **thinking:** Olhar direcionado para cima por sobre a armação, sobrancelha direita erguida, expressão reflexiva.
6. **angry:** Sobrancelhas franzidas em V incisivo sobre os aros, pupilas focadas, lábios cerrados com firmeza.
7. **sad:** Olhar cabisbaixo por trás das lentes, cantos da boca caídos sob o bigode, postura melancólica.
8. **scared:** Olhos arregalados e redondos através das lentes, boca entreaberta em susto.
