# Especificação Técnica — Recriação da Billi no Pipeline Autêntico do Comic Chat (#30)

- **ID do Ticket:** #30
- **Status:** Em Implementação
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Comic Chat (Jim Woodring)

## 1. Contexto & Diagnóstico da Discrepância Estética
Na implementação do Ticket #24, a personagem **Billi** foi construída utilizando primitivas vetoriais SVG manuais (`<path>`, `<polygon>`, `<circle>`, `<rect>`) com preenchimentos de cores hexadecimais chapadas (tons de cinza, bege, marrom). O resultado visual foi um traçado geométrico simplificado e artificial que destoava radicalmente do restante do elenco clássico (especialmente do **Arti**).

Ao inspecionar a arquitetura de assets do **Arti**, identificou-se o verdadeiro pipeline estético que confere ao personagem a riqueza orgânica do traço de Jim Woodring no Microsoft Comic Chat (1996):
1. **Padrão Gráfico Comic Chat:** Em vez de polígonos vetoriais estilizados, os personagens originais do Comic Chat eram bitmaps de nanquim e hachura de altíssimo contraste com exatamente três componentes de cor:
   - `Transparente (A = 0)`: Exterior ao corpo do personagem, permitindo a visualização limpa do cenário de fundo.
   - `Preto Sólido (0, 0, 0, 255)`: Linhas de nanquim, tramas de hachura, dobras de tecido, fios de cabelo, traços anatômicos e contornos de alto contraste derivados da digitalização/processamento da imagem real.
   - `Branco Sólido (255, 255, 255, 255)`: A lendária *nimbus mask* (máscara de oclusão do corpo) que bloqueia os elementos do cenário que passam atrás da figura humana, criando a ilusão de opacidade sem necessidade de cores digitais artificiais.
2. **Encapsulamento SVG:** A imagem em três estados (transparente, branco nimbus, preto nanquim) é serializada em PNG otimizado e incorporada via `<image href="data:image/png;base64,..." />` dentro do grupo canônico `<g class="billi-ink billi-{pose}">` com `viewBox="0 0 160 400"`.

## 2. Decisão Arquitetural para a Natasha
**Decisão:** Abandonar completamente as primitivas vetoriais manuais na Billi e aplicar rigorosamente o pipeline de processamento de imagem em nanquim e hachuras do Comic Chat a partir do acervo fotográfico real em `~/ref/billi/`:
- **Foto de Corpo Inteiro (`perfil.jpg`):** Base para silhueta, postura de alfaiataria (blazer de lapela estruturada, calça reta de alfaiataria, botas pretas de couro com salto bloco, decote em U e proporções anatômicas).
- **Closes Faciais e Detalhes de Identidade (`~/ref/billi`):**
  - Cabelo pixie repicado e desfiado com mechas características.
  - Tatuagem de mariposa com asas abertas no colo em nanquim detalhado.
  - Piercings faciais precisos: bridge na ponte do nariz entre os olhos, transversal na sobrancelha esquerda, nostril e labiais.
  - Delineador marcante nos olhos.
- **Diferenciação Nítida das 8 Poses Canônicas:**
  - `neutral`: Postura ereta, olhar atento, sorriso sutil confiante.
  - `happy`: Sorriso aberto mostrando dentes, bochechas levantadas, olhos brilhantes em arco.
  - `laughing`: Gargalhada aberta inspirada na selfie do espelho, cabeça levemente inclinada, olhos semicerrados de rir.
  - `shouting`: Projeção de boca aberta em fala enérgica, sobrancelhas arqueadas.
  - `thinking`: Olhar para cima, sobrancelha direita erguida, expressão reflexiva (baseada na foto `b87ccca4`).
  - `angry`: Sobrancelhas franzidas em V marcante sobre a ponte nasal, lábios cerrados com rigidez.
  - `sad`: Olhar cabisbaixo, sobrancelhas inclinadas em melancolia, cantos da boca caídos.
  - `scared`: Olhos arregalados circulares, boca entreaberta em sobressalto.

## 3. Critérios de Aceitação e Testes Automatizados
1. Todos os 8 arquivos SVG em `assets/characters/billi/*.svg` devem conter o grupo `<g class="billi-ink billi-{pose}">` e a imagem embutida com nimbus mask sólida branca e nanquim preto.
2. Nenhuma cor chapada de preenchimento artificial (como `#1e232a`, `#f8eedf`, etc.) deve existir nos assets da Billi.
3. As 8 poses devem possuir variação fisionômica mensurável (diferença de pixels > 0 entre poses consecutivas).
4. O `manifest.json` da Billi deve registrar as 8 poses e as âncoras anatômicas de balão e mão.
5. O renderizador Ayvu e os testes de conformidade estética do Comic Chat devem passar sem regressões.
