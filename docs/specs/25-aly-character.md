# Especificação Técnica — Recriação da Personagem Aly (#25)

- **ID do Ticket:** #25
- **Status:** Em Implementação
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Vetorial / Fotos de Referência (`~/ref/aly`)

## 1. Contexto & Árvore de Decisão
No marco do Ticket #22, o elenco nativo de Ayvu introduziu a personagem **Aly** como uma silhueta feminina preenchida em preto com estampa circular provisória contendo a letra "A" no peito. Essa solução de placeholder violava os padrões estéticos definidos para o projeto:
1. **Placeholder Artificial:** A estampa circular com a letra "A" era um remendo digital provisório sem personalidade nem traço autêntico de história em quadrinhos.
2. **Incompatibilidade com o Padrão do Comic Chat:** O projeto Ayvu exige nanquim denso, hachuras orgânicas e expressividade anatômica.
3. **Ausência de Fidelidade com a Referência Real:** As fotos em `~/ref/aly` fornecem a identidade canônica completa da Aly — sua expressão alegre, cabelo cacheado volumoso, jaqueta esportiva casual com zíper, calça jeans e tênis de lona.

**Decisão para a Natasha:**
Substituir todas as 8 poses da Aly por ilustrações vetoriais em nanquim autêntico baseadas fielmente nas fotografias de `~/ref/aly`:
- `perfil.jpg`: corpo inteiro, vestimenta casual (jaqueta esportiva cinza-azulada com zíper, jeans clássico, tênis estilo canvas com biqueira de borracha) e proporção anatômica.
- `IMG_20260310_162131 (1).jpg`, `IMG_20260311_152912 (2).jpg` e `IMG_20260311_152913 (1) (1).jpg`: estrutura facial, sobrancelhas arqueadas e expressivas, formato dos olhos, sorriso acolhedor e cachos volumosos na altura dos ombros.

## 2. Identidade Visual e Anatomia Vetorial

### 2.1 Vestimenta & Elementos de Design
- **Jaqueta Casual:** Tom azul-acinzentado (`#526b7d`) com contornos firmes em nanquim (`stroke="#111111" stroke-width="1.8"`), zíper central metálico (`#d1d5db`), costura horizontal no tórax e punhos ajustados.
- **Cordão e Óculos:** Fita de pescoço pendente sobre a jaqueta com óculos de sol presos discretamente à gola.
- **Calça Jeans:** Jeans escuro (`#273b50`), corte reto, com vincos leves nos joelhos e barra ajustada.
- **Tênis Canvas (All-Star):** Tênis de lona bege/cinza claro (`#9b9281`), biqueira branca de borracha (`#f3f4f6`), ilhoses e cadarços brancos.

### 2.2 Fisionomia, Cabelo & Expressividade
- **Cabelo:** Cachos volumosos castanho-escuros (`#2a1f18`) com mechas espiraladas e hachuras em nanquim, caimento até os ombros e divisão central suave.
- **Rosto:** Traços faciais gentis, maçãs do rosto suaves, sobrancelhas expressivas e arqueadas, olhos amendoados e dentes à mostra nos sorrisos (`happy`, `laughing`).
- **Acessórios:** Pequenas argolas douradas nos lóbulos das orelhas.

### 2.3 Âncoras Anatômicas
- `balloon_anchor`: `(0.50, 0.18)` — centralizado acima da coroa dos cachos para balões de fala, grito e pensamento.
- `hand_anchor`: `(0.68, 0.49)` — alinhado com precisão na mão direita anatômica (posição onde segura objetos e props como canecas, livros, microfone).

## 3. As 8 Poses Canônicas
1. **neutral:** Em pé, postura relaxada e acolhedora, sorriso sereno, braços ao lado do corpo.
2. **happy:** Sorriso aberto mostrando dentes, olhos expressivos brilhando, mão direita erguida em saudação amistosa.
3. **laughing:** Gargalhada aberta e contagiante, cabeça levemente inclinada, olhos fechados em arco alegre.
4. **shouting:** Boca aberta projetando fala/grito, sobrancelhas erguidas, mão em concha junto à boca.
5. **thinking:** Olhar voltado para cima e para a esquerda, dedo indicador no queixo/bochecha, boca em expressão reflexiva.
6. **angry:** Sobrancelhas franzidas com foco, braços cruzados sobre o tórax da jaqueta, lábios cerrados.
7. **sad:** Olhar cabisbaixo, sobrancelhas arqueadas internamente em melancolia, ombros caídos.
8. **scared:** Olhos arregalados, boca em "O", mãos espalmadas à frente do peito em reação de sobressalto.
