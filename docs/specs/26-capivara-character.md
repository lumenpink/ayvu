# Especificação Técnica — Recriação da Capivara em Nanquim Autêntico (#26)

- **ID do Ticket:** #26
- **Status:** Em Implementação
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Comic Chat (Jim Woodring) / Fotos de Referência (`~/ref/capivara`)

## 1. Contexto & Árvore de Decisão
A implementação inicial da Capivara no Ticket #19 utilizou formas vetoriais simplificadas com preenchimentos marrons chapados (`#2b231d`, `#3a2f27`, `#4a3c32`). Essa abordagem causou uma disparidade estética crítica em relação ao personagem Arti, que segue com fidelidade a textura e o contraste do Microsoft Comic Chat (1996).

**Decisão para a Natasha:**
Adotar o mesmo pipeline de nanquim autêntico do Comic Chat utilizado na criação do Arti:
1. Purgar completamente preenchimentos coloridos marrons e traços geométricos de Paint.
2. Utilizar as fotografias reais de alta resolução em `~/ref/capivara/` (especialmente `Bristol.zoo.capybara.arp.jpg` e `Capivara(Hydrochoerus_hydrochaeris).jpg`).
3. Aplicar extração anatômica: crânio quadrado, narinas proeminentes, orelhas pequenas arredondadas, patas robustas com dedos separados e textura densa de cerdas/pêlos em hachura de nanquim preto e branco de alto contraste com máscara de silhueta.
4. Gerar as 8 poses canônicas com variações expressivas reais de boca, olhar e postura.

## 2. Poses Canônicas & Expressividade
1. **neutral:** Capivara em postura calma clássica, focinho reto e olhar tranquilo.
2. **happy:** Focinho levemente erguido, olhos semicerrados em contentamento e boca em leve curvatura de sorriso.
3. **laughing:** Boca aberta mostrando os incisivos característicos, olhos fechados em arco alegre.
4. **shouting:** Cabeça projetada para frente, narinas dilatadas, boca aberta em vocalização / latido de capivara.
5. **thinking:** Olhar inclinado para cima, orelha curvada em dúvida, focinho reflexivo.
6. **angry:** Cerdas do dorso arrepiadas, olhar tenso focado e postura defensiva.
7. **sad:** Focinho apontando para baixo, orelhas caídas e olhar melancólico.
8. **scared:** Olhos esbugalhados, orelhas em alerta máximo e corpo em sobressalto.

## 3. Âncoras Anatômicas
- `balloon_anchor`: `(0.50, 0.15)` — no topo da cabeça/orelhas para fixação dos balões.
- `hand_anchor`: `(0.70, 0.65)` — na pata dianteira anatômica para apoiar props.
