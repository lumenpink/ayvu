# Especificação Técnica — Recriação do Jacaré em Nanquim Autêntico (#27)

- **ID do Ticket:** #27
- **Status:** Em Implementação
- **Autora:** Lumen
- **Alinhamento:** GOST 1.4.0 / Ayvu Core v2.0 / Padrão Nanquim Comic Chat (Jim Woodring) / Fotos de Referência (`~/ref/jacare`)

## 1. Contexto & Árvore de Decisão
O jacaré do pantanal/manguezal da UFSC (*Caiman latirostris* — jacaré-do-papo-amarelo) havia sido implementado com preenchimentos de cor verde e amarelo no Ticket #19. Assim como a Capivara, apresentava inconsistência visual com a estética em nanquim autêntico de alta fidelidade do Comic Chat.

**Decisão para a Natasha:**
Aplicar o mesmo pipeline de nanquim do Comic Chat (Jim Woodring) a partir das fotos reais de referência em `~/ref/jacare/` (`Caiman_latirostris.jpg` e `Caiman_latirostris_Oliwa.JPG`):
1. Purgar completamente os preenchimentos verdes e amarelos (`#243823`, `#2d472c`, `#c9a84a`, `#f0c808`).
2. Adotar contraste estrito em preto e branco com hachuras densas de escamas e placas ósseas (osteodermos).
3. Fidelidade anatômica do jacaré-de-papo-amarelo: focinho em "U" largo e arredondado, dentes visíveis com a boca fechada, crista ocular elevada e cauda denteada.
4. Produzir 8 poses expressivas em conformidade com as convenções narrativas do Comic Chat.

## 2. Expressividade das 8 Poses Canônicas
1. **neutral:** Mandíbula serrilhada fechada com dentes cônicos à mostra, olhar réptil atento.
2. **happy:** Olhos arqueados em semicírculo relaxado, ponta do focinho suavemente inclinada para cima em sorriso réptil.
3. **laughing:** Focinho escancarado exibindo a fileira completa de dentes pontiagudos e língua interna em hachura.
4. **shouting:** Cabeça projetada e mandíbulas abertas em vocalização territorial de réptil, cristas eriçadas.
5. **thinking:** Olhar voltado para o céu, cabeça ligeiramente lateralizada, crista arqueada.
6. **angry:** Cristas ósseas proeminentes, pupila em fenda vertical fina, postura tensa e osteodermos dorsais destacados.
7. **sad:** Focinho cabisbaixo, pálpebras caídas sobre os olhos e dentes retraídos.
8. **scared:** Olhos arredondados espantados, mandíbula entreaberta trêmula.

## 3. Âncoras Anatômicas
- `balloon_anchor`: `(0.50, 0.18)` — no topo da cabeça réptil.
- `hand_anchor`: `(0.68, 0.55)` — na pata dianteira com garras palmadas para manusear props.
